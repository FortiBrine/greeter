use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
}

pub struct DatabaseModule {
    pub conn: Mutex<Connection>,
    path: PathBuf,
}

impl DatabaseModule {
    pub async fn open(path: PathBuf) -> Result<Self, DbError> {
        let path_clone = path.clone();
        let conn = tokio::task::spawn_blocking(move || Connection::open(&path_clone)).await??;
        Ok(Self { conn: Mutex::new(conn), path })
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }
}
