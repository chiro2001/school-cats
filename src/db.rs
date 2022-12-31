use anyhow::{anyhow, Result};
use log::*;
use mysql::*;
use mysql::prelude::*;

pub const SQL_FILE: &'static str = "database/crebas.sql";

pub fn sql_url() -> String {
    std::env::var("DATABASE_URL").unwrap().as_str().to_string()
}

pub async fn db_init(pool: &Pool) -> Result<()> {
    let content = std::fs::read_to_string(SQL_FILE)?;
    let split = content.split(";").collect::<Vec<&str>>();
    let mut conn = pool.get_conn().unwrap().unwrap();
    for s in split {
        let t = s.trim();
        if t.is_empty() {
            continue;
        }
        let exec = format!("{};", t);
        info!("exec: {}", exec);
        let r: Vec<String> = match conn.exec(exec, Params::Empty) {
            Ok(r) => r,
            Err(e) => vec![format!("{:?}", e)],
        };
        info!("db: {:?}", r);
    }
    Ok(())
}

pub async fn db_get_pool() -> Result<Pool> {
    let pool = Pool::new(sql_url().as_str()).unwrap();
    match std::env::var("DB_FORCE_INIT") {
        Ok(s) => match s.as_str() {
            "1" => {
                db_init(&pool).await?;
            }
            _ => {}
        }
        Err(e) => {}
    };
    Ok(pool)
}