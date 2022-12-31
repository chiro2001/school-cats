use anyhow::{anyhow, Result};
use sqlx::Pool;
use log::*;
use sqlx::migrate::MigrateDatabase;

pub enum SupportedSQL {
    MYSQL,
    SQLITE,
}

pub const SQL_USE: SupportedSQL = SupportedSQL::SQLITE;
pub const DB_FORCE_INIT: bool = true;

type DB = sqlx::Sqlite;

pub const SQLITE_PATH: &str = "database/sqlite.db";
pub const SQL_FILE: &str = "database/crebas.sql";

pub fn sqlite_path() -> String {
    std::env::var("DATABASE_URL").unwrap().as_str().to_string()
}

async fn db_conn_sqlite() -> Result<Pool<DB>> {
    std::env::set_var("DATABASE_URL", sqlite_path());
    if DB_FORCE_INIT {
        warn!("init database {}", sqlite_path());
        if std::path::Path::new(SQLITE_PATH).exists() {
            std::fs::remove_file(SQLITE_PATH)?;
        }
        sqlx::Sqlite::create_database(&sqlite_path()).await?;
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&sqlite_path()).await?;
        sqlx::query_file!("database/crebas.sql").execute(&pool).await?;
        Ok(pool)
    } else {
        Ok(sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&sqlite_path()).await?)
    }
}

pub async fn db_conn() -> Result<Pool<DB>> {
    let pool = match SQL_USE {
        // SupportedSQL::MYSQL => {
        //     sqlx::mysql::MySqlPoolOptions::new()
        //         .max_connections(5)
        //         .connect("mysql://127.0.0.1:3306/school_cats?user=root").await
        // }
        SupportedSQL::SQLITE => {
            db_conn_sqlite().await
        }
        _ => panic!("unsupported")
    };
    let pool = match pool {
        Ok(p) => Ok(p),
        Err(e) => {
            error!("cannot connect to pool: {:?}, retry", e);
            db_conn_sqlite().await
        }
    };
    match pool {
        Ok(p) => Ok(p),
        Err(e) => Err(anyhow!("database connect failed! {:?}", e))
    }
}