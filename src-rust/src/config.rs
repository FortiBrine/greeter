use std::path::Path;

use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../../src/main/resources/config.yml");

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

impl Config {
    pub async fn load_or_create(folder: &Path) -> Result<Self, ConfigError> {
        tokio::fs::create_dir_all(folder).await?;

        let config_path = folder.join("config.yml");
        if !config_path.exists() {
            tokio::fs::write(&config_path, DEFAULT_CONFIG).await?;
        }

        let contents = tokio::fs::read_to_string(&config_path).await?;
        Ok(serde_yaml::from_str(&contents)?)
    }
}
