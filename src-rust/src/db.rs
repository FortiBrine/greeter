use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;

pub struct DatabaseModule {
    pub conn: Mutex<Connection>,
    path: PathBuf,
}

impl DatabaseModule {
    pub async fn open(path: PathBuf) -> anyhow::Result<Self> {
        let path_clone = path.clone();
        let conn = tokio::task::spawn_blocking(move || Connection::open(&path_clone)).await??;
        Ok(Self { conn: Mutex::new(conn), path })
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }
}
