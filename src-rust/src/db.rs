use std::path::{Path, PathBuf};

use sea_query::{
    Expr, ExprTrait, Iden, MysqlQueryBuilder, OnConflict, Query, QueryStatementBuilder,
    SqliteQueryBuilder,
};
use sqlx::AnyPool;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("migration failed: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

#[derive(Iden)]
pub(crate) enum PlayerGreets {
    Table,
    Uuid,
    Message,
}

enum Driver {
    Sqlite(PathBuf),
    Mysql,
}

impl Driver {
    fn sql<S: QueryStatementBuilder>(&self, stmt: &S) -> String {
        match self {
            Driver::Sqlite(_) => stmt.build_any(&SqliteQueryBuilder).0,
            Driver::Mysql => stmt.build_any(&MysqlQueryBuilder).0,
        }
    }
}

pub struct Database {
    pool: AnyPool,
    driver: Driver,
}

impl Database {
    pub async fn connect_sqlite(path: &Path) -> Result<Self, DbError> {
        sqlx::any::install_default_drivers();
        let url = format!("sqlite:{}", path.to_str().expect("non-UTF-8 path"));
        let pool = AnyPool::connect(&url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool, driver: Driver::Sqlite(path.to_path_buf()) })
    }

    pub async fn connect_mysql(url: &str) -> Result<Self, DbError> {
        sqlx::any::install_default_drivers();
        let pool = AnyPool::connect(url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool, driver: Driver::Mysql })
    }

    pub fn connection_info(&self) -> String {
        match &self.driver {
            Driver::Sqlite(path) => format!("SQLite at {}", path.display()),
            Driver::Mysql => "MySQL".to_string(),
        }
    }

    pub async fn get_greet(&self, uuid: &str) -> Result<Option<String>, DbError> {
        let sql = self.driver.sql(
            Query::select()
                .column(PlayerGreets::Message)
                .from(PlayerGreets::Table)
                .and_where(Expr::col(PlayerGreets::Uuid).eq(uuid)),
        );
        Ok(sqlx::query_scalar(&sql).bind(uuid).fetch_optional(&self.pool).await?)
    }

    pub async fn set_greet(&self, uuid: &str, message: &str) -> Result<(), DbError> {
        let sql = self.driver.sql(
            Query::insert()
                .into_table(PlayerGreets::Table)
                .columns([PlayerGreets::Uuid, PlayerGreets::Message])
                .values_panic([uuid.into(), message.into()])
                .on_conflict(
                    OnConflict::column(PlayerGreets::Uuid)
                        .update_column(PlayerGreets::Message)
                        .to_owned(),
                ),
        );
        sqlx::query(&sql).bind(uuid).bind(message).execute(&self.pool).await?;
        Ok(())
    }
}
