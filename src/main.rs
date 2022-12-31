mod db;

use warp::Filter;
use cats_api::*;
use crate::db::db_conn;
use anyhow::Result;
use log::info;

// fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
//     // When accepting a body, we want a JSON body
//     // (and to reject huge payloads)...
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    info!("school-cats backend!");
    let pool = db_conn().await?;
    info!("server running on http://127.0.0.1:{}/", PORT);
    let cors = warp::cors().allow_any_origin();
    let hello = warp::path!("hello" / String)
        .map(|name: String| serde_json::to_string(&Hello { msg: name.to_string() }).unwrap())
        .with(cors);

    // let get_user = warp::get()
    //     .and(warp::path("user"))
    //     .and(warp::path::end())
    //     .and(json_body())

    warp::serve(hello).run(([127, 0, 0, 1], PORT)).await;
    Ok(())
}
