use crate::config::ConfigError;
use crate::db::DbError;

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error("failed to create Tokio runtime: {0}")]
    Runtime(std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Db(#[from] DbError),
}
