#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::time::SystemTime;
use anyhow::{anyhow, Result};
use encoding::{DecoderTrap, Encoding};
use log::*;
use mysql::*;
use mysql::prelude::*;
use cats_api::jwt::TokenDB;
use cats_api::user::UserDB;

pub const SQL_FILE: &'static str = "database/crebas.sql";

pub fn sql_url() -> String {
    std::env::var("DATABASE_URL").unwrap().as_str().to_string()
}

pub async fn db_init(pool: &Pool) -> Result<()> {
    let content_utf8 = std::fs::read_to_string(SQL_FILE);
    let split = match content_utf8 {
        Ok(content) => content.split(";").map(|s| s.to_string()).collect::<Vec<String>>(),
        Err(_) => {
            warn!("retry reading using gbk");
            let file = File::open(SQL_FILE)?;
            let reader = BufReader::new(&file);
            let queries = reader.split(b';').map(|q| q.unwrap());
            queries.map(|q| encoding::all::GBK.decode(&q, DecoderTrap::Strict).unwrap())
                .collect::<Vec<String>>()
        }
    };
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
    conn.exec_drop("SET FOREIGN_KEY_CHECKS=1;", Params::Empty)?;
    // insert default image
    conn.query_drop("INSERT INTO Image (url) VALUES (\"https://yew.rs/img/logo.svg\")")?;
    assert_eq!(1, conn.last_insert_id());
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
    pub fn create_token(&self, uid: u32, exp: SystemTime) -> Result<String> {

    }
    pub fn user(&self, uid: u32) -> Result<UserDB> {
        let mut conn = self.conn()?;
        let r = conn.exec_first("SELECT userId,username,imageId,usernick,motto FROM User WHERE uid = :uid",
                                params! { "uid" => uid })
            .map(|row| {
                row.map(|(userId, username, imageId, usernick, motto)| UserDB {
                    userId,
                    username,
                    imageId,
                    usernick,
                    motto,
                    passwd: "".to_string(),
                })
            })?;
        match r {
            Some(t) => Ok(t),
            None => Err(anyhow!("no uid found for {}", uid))
        }
    }
    pub fn image_insert(&self, url: &str) -> Result<u32> {
        let mut conn = self.conn()?;
        conn.exec_drop("INSERT INTO Image (url) VALUES (:url);", params! { "url" => url })?;
        Ok(conn.last_insert_id() as u32)
    }
    pub fn user_insert(&self, user: UserDB) -> Result<u32> {
        let mut conn = self.conn()?;
        info!("insert user: {:?}", user);
        conn.exec_drop("INSERT INTO User (username,passwd,imageId,usernick,motto) VALUES (?,?,?,?,?);", (
            user.username,
            user.passwd,
            user.imageId,
            user.usernick,
            user.motto
        ))?;
        Ok(conn.last_insert_id() as u32)
    }
}