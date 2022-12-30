use anyhow::{anyhow, Result};
use sqlx::{Database, Pool};
use log::*;
use sqlx::migrate::MigrateDatabase;

pub enum SupportedSQL {
    MYSQL,
    SQLITE,
}

pub const SQL_USE: SupportedSQL = SupportedSQL::SQLITE;

type DB = sqlx::Sqlite;

pub const SQLITE_PATH: &str = "database/sqlite.db";

pub async fn db_conn() -> Result<Pool<DB>> {
    let pool = match SQL_USE {
        // SupportedSQL::MYSQL => {
        //     sqlx::mysql::MySqlPoolOptions::new()
        //         .max_connections(5)
        //         .connect("mysql://127.0.0.1:3306/school_cats?user=root").await
        // }
        SupportedSQL::SQLITE => {
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(SQLITE_PATH).await
        }
        _ => panic!("unsupported")
    };
    let pool = match pool {
        Ok(p) => Ok(p),
        Err(e) => {
            error!("cannot connect to pool: {:?}, trying to init database", e);
            sqlx::Sqlite::create_database(SQLITE_PATH).await?;
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(SQLITE_PATH).await
        }
    };
    match pool {
        Ok(p) => Ok(p),
        Err(e) => Err(anyhow!("database connect failed! {:?}", e))
    }
}