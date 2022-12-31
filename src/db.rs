use std::sync::Arc;
use std::time::SystemTime;
use anyhow::{anyhow, Result};
use log::*;
use mysql::*;
use mysql::prelude::*;
use cats_api::jwt::TokenDB;

pub const SQL_FILE: &'static str = "database/crebas.sql";

pub fn sql_url() -> String {
    std::env::var("DATABASE_URL").unwrap().as_str().to_string()
}

pub async fn db_init(pool: &Pool) -> Result<()> {
    let content = std::fs::read_to_string(SQL_FILE)?;
    let split = content.split(";").collect::<Vec<&str>>();
    let mut conn = pool.get_conn().unwrap().unwrap();
    info!("SET FOREIGN_KEY_CHECKS=0;");
    let _: Vec<String> = conn.exec("SET FOREIGN_KEY_CHECKS=0;", Params::Empty)?;
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
    info!("SET FOREIGN_KEY_CHECKS=1;");
    let _: Vec<String> = conn.exec("SET FOREIGN_KEY_CHECKS=1;", Params::Empty)?;
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
        Err(_e) => {}
    };
    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<Pool>,
}

impl Database {
    pub fn new(pool: Pool) -> Self {
        Self { pool: Arc::new(pool) }
    }
    pub fn conn(&self) -> Result<PooledConn> {
        match self.pool.get_conn() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(anyhow!("cannot get conn: {:?}", e))
        }
    }
    pub fn token_check(&self, token: &str) -> Result<TokenDB> {
        let mut conn = self.conn()?;
        let r = conn.exec_first("SELECT token,uid FROM Token WHERE token = :token",
                                params! { "token" => token })
            .map(|row| {
                row.map(|(token, uid)| TokenDB {
                    token,
                    exp: SystemTime::now(),
                    uid,
                })
            })?;
        match r {
            Some(t) => Ok(t),
            None => Err(anyhow!("no token found for {}", token))
        }
    }
}