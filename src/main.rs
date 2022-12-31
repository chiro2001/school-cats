mod db;

use warp::Filter;
use cats_api::*;
use crate::db::db_get_pool;
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

// fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
//     // When accepting a body, we want a JSON body
//     // (and to reject huge payloads)...
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub a: usize,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Empty;

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub code: usize,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(code: usize, msg: &str, data: T) -> Self {
        Self { code, msg: msg.to_string(), data }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    info!("school-cats backend!");
    let _pool = db_get_pool().await?;
    info!("server running on http://127.0.0.1:{}/", PORT);
    let cors = warp::cors().allow_any_origin();
    let index = warp::path::end().map(|| warp::reply::json(&Response::new(200, "api root", Empty::default())));
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param::<String>())
        // .and(warp::body::content_length_limit(1024 * 16))
        // .and(warp::body::json())
        .map(|name: String| warp::reply::json(
            // &Hello { msg: name.to_string() }
            &Response { code: 200, msg: "ok".to_string(), data: Data { a: name.len() } }
        ));
    let routes = warp::any().and(
        index
            .or(hello)
            .with(cors)
    );

    // let get_user = warp::get()
    //     .and(warp::path("user"))
    //     .and(warp::path::end())
    //     .and(json_body())

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
    Ok(())
}
