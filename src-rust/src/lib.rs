mod config;
mod db;
mod error;

use std::path::PathBuf;
use std::sync::OnceLock;

use config::{Config, ConfigError, DatabaseDriver};
use db::Database;
pub use error::PluginError;
use jni::objects::{JObject, JString};
use jni::{jni_mangle, EnvUnowned};
use tokio::runtime::Runtime;

static PLUGIN: OnceLock<GreeterPlugin> = OnceLock::new();

struct GreeterPlugin {
    runtime: Runtime,
    db: Database,
    config: Config,
}

impl GreeterPlugin {
    async fn init(folder: &str) -> Result<(Database, Config), PluginError> {
        let folder_path = PathBuf::from(folder);
        let config = Config::load_or_create(&folder_path).await?;

        let db = match config.database.driver {
            DatabaseDriver::Sqlite => {
                let path = folder_path
                    .join(config.database.path.as_deref().unwrap_or("database.db"));
                Database::connect_sqlite(&path).await?
            }
            DatabaseDriver::Mysql => {
                let url = config
                    .database
                    .url
                    .as_deref()
                    .ok_or(ConfigError::MissingField("database.url"))?;
                Database::connect_mysql(url).await?
            }
        };

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

        let msg = format!("Connected to {}", db.connection_info());
        PLUGIN.get_or_init(|| GreeterPlugin { runtime, db, config });

        Ok(JString::from_str(env, &msg)?)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[jni_mangle("me.fortibrine.greeter.RustBridge")]
pub fn onPlayerJoin<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _: JObject,
    uuid: JString<'caller>,
) -> JString<'caller> {
    let outcome = unowned_env.with_env(|env| -> Result<_, PluginError> {
        let uuid = uuid.try_to_string(env)?;
        let p = plugin();
        let greet = p.runtime.block_on(p.db.get_greet(&uuid))?;
        Ok(JString::from_str(env, greet.as_deref().unwrap_or(""))?)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[jni_mangle("me.fortibrine.greeter.RustBridge")]
pub fn onGreetCommand<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _: JObject,
    uuid: JString<'caller>,
    message: JString<'caller>,
) -> JString<'caller> {
    let outcome = unowned_env.with_env(|env| -> Result<_, PluginError> {
        let uuid = uuid.try_to_string(env)?;
        let message = message.try_to_string(env)?;
        let p = plugin();
        let response = format!("Greet message set to: {message}");
        p.runtime.spawn(async move {
            if let Err(e) = p.db.set_greet(&uuid, &message).await {
                eprintln!("[Greeter] set_greet failed: {e}");
            }
        });
        Ok(JString::from_str(env, &response)?)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
