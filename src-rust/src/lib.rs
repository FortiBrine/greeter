mod config;
mod db;
mod error;

use std::path::PathBuf;
use std::sync::OnceLock;

use config::Config;
use db::DatabaseModule;
pub use error::PluginError;
use jni::objects::{JObject, JString};
use jni::{jni_mangle, EnvUnowned};
use tokio::runtime::Runtime;

static PLUGIN: OnceLock<GreeterPlugin> = OnceLock::new();

struct GreeterPlugin {
    runtime: Runtime,
    db: DatabaseModule,
    config: Config,
}

impl GreeterPlugin {
    async fn init(folder: &str) -> Result<(DatabaseModule, Config), PluginError> {
        let folder_path = PathBuf::from(folder);
        let config = Config::load_or_create(&folder_path).await?;
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

        let runtime = Runtime::new().map_err(PluginError::Runtime)?;
        let (db, config) = runtime.block_on(GreeterPlugin::init(&folder))?;

        let msg = format!("Connected to SQLite at {}", db.path_display());
        PLUGIN.get_or_init(|| GreeterPlugin { runtime, db, config });

        Ok(JString::from_str(env, &msg)?)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
