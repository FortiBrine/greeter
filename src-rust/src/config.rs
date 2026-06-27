use std::path::Path;

use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../../src/main/resources/config.yml");

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_saphyr::Error),
    #[error("missing required config field: {0}")]
    MissingField(&'static str),
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseDriver {
    Sqlite,
    Mysql,
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub driver: DatabaseDriver,
    pub path: Option<String>,
    pub url: Option<String>,
}

impl Config {
    pub async fn load_or_create(folder: &Path) -> Result<Self, ConfigError> {
        tokio::fs::create_dir_all(folder).await?;
        let config_path = folder.join("config.yml");
        if !config_path.exists() {
            tokio::fs::write(&config_path, DEFAULT_CONFIG).await?;
        }
        let contents = tokio::fs::read_to_string(&config_path).await?;
        Ok(serde_saphyr::from_str(&contents)?)
    }
}
