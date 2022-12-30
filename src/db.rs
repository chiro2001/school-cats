use anyhow::{anyhow, Result};
use sqlx::{Database, Pool};

pub enum SupportedSQL {
    MYSQL,
    SQLITE,
}

pub const SQL_USE: SupportedSQL = SupportedSQL::SQLITE;
type DB = sqlx::Sqlite;
pub const SQLITE_PATH: &str = "database/sqlite.db";

pub async fn db_init() -> Result<Pool<DB>> {
    let pool = match SQL_USE {
        // SupportedSQL::MYSQL => {
        //     sqlx::mysql::MySqlPoolOptions::new()
        //         .max_connections(5)
        //         .connect("mysql://127.0.0.1:3306/school_cats?user=root").await
        // }
        SupportedSQL::SQLITE => {
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect("database/sqlite.db").await
        }
        _ => panic!("unsupported")
    };
    // sqlx::query_as("SELECT $1").bind(150_i64)
    match pool {
        Ok(p) => Ok(p),
        Err(e) => Err(anyhow!("database connect failed! {:?}", e))
    }
}