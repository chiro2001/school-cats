use anyhow::{anyhow, Result};
use log::*;
use mysql::*;
use mysql::prelude::*;

pub fn sqlite_path() -> String {
    std::env::var("DATABASE_URL").unwrap().as_str().to_string()
}

pub async fn db_conn() -> Result<Pool<DB>> {

}