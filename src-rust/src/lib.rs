mod config;
mod db;

use std::path::PathBuf;
use std::sync::OnceLock;

use config::Config;
use db::DatabaseModule;
use jni::objects::{JObject, JString};
use jni::{jni_mangle, EnvUnowned};
use tokio::runtime::Runtime;

const DEFAULT_CONFIG: &str = include_str!("../../src/main/resources/config.yml");

static PLUGIN: OnceLock<GreeterPlugin> = OnceLock::new();

/// Concrete error type for the JNI boundary.
/// `ThrowRuntimeExAndDefault` requires `E: std::error::Error`, which neither
/// `Box<dyn Error>` nor `anyhow::Error` satisfies due to Sized/design constraints.
#[derive(Debug)]
struct PluginError(String);

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PluginError {}

impl From<jni::errors::Error> for PluginError {
    fn from(e: jni::errors::Error) -> Self {
        Self(e.to_string())
    }
}

struct GreeterPlugin {
    runtime: Runtime,
    db: DatabaseModule,
    config: Config,
}

impl GreeterPlugin {
    async fn init(folder: &str) -> anyhow::Result<(DatabaseModule, Config)> {
        let folder_path = PathBuf::from(folder);
        tokio::fs::create_dir_all(&folder_path).await?;

        let config_path = folder_path.join("config.yml");
        if !config_path.exists() {
            tokio::fs::write(&config_path, DEFAULT_CONFIG).await?;
        }

        let config = Config::load(&config_path).await?;
        let db_path = folder_path.join(&config.database.path);
        let db = DatabaseModule::open(db_path).await?;

        Ok((db, config))
    }
}

fn plugin() -> &'static GreeterPlugin {
    PLUGIN.get().expect("GreeterPlugin not initialized")
}

#[jni_mangle("me.fortibrine.greeter.RustBridge")]
pub fn on_enable<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _: JObject,
    data_folder: JString<'caller>,
) -> JString<'caller> {
    let outcome = unowned_env.with_env(|env| -> Result<_, PluginError> {
        let folder = data_folder.try_to_string(env)?;

        let runtime = Runtime::new().map_err(|e| PluginError(e.to_string()))?;
        let (db, config) = runtime
            .block_on(GreeterPlugin::init(&folder))
            .map_err(|e| PluginError(e.to_string()))?;

        let msg = format!("Connected to SQLite at {}", db.path_display());
        PLUGIN.get_or_init(|| GreeterPlugin { runtime, db, config });

        Ok(JString::from_str(env, &msg)?)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
