mod db;

use warp::{Filter, http};
use cats_api::*;
use crate::db::db_get_pool;
use anyhow::Result;
use log::info;
use warp::http::Method;
use cats_api::user::{LoginPost, LoginResponse, User};

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
    let cors = warp::cors().allow_any_origin().allow_methods(vec![
        Method::OPTIONS, Method::GET, Method::POST, Method::DELETE, Method::PUT,
    ]).allow_headers(vec![
        http::header::CONTENT_TYPE, http::header::ORIGIN,
        http::header::HeaderName::from_static("token"),
        http::header::HeaderName::from_static("refresh-token"),
    ]);
    let index = warp::path::end().map(|| warp::reply::json(&Response::new(200, "api root", Empty::default())));
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param::<String>())
        // .and(warp::body::content_length_limit(1024 * 16))
        // .and(warp::body::json())
        .map(|name: String| warp::reply::json(
            &Response::ok(Hello { msg: name })
        ));

    let user_get = warp::get()
        .and(warp::path("user"))
        .and(warp::path::param::<u32>())
        .map(|_uid| warp::reply::json(&Response::ok(User::default())));

    let login_post = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .map(|r: LoginPost| warp::reply::json(&Response::ok(LoginResponse { token: r.username, refresh_token: "".to_string() })));


    let routes = warp::any().and(
        index
            .or(hello)
            .or(user_get)
            .or(login_post)
    ).with(cors);

    // let get_user = warp::get()
    //     .and(warp::path("user"))
    //     .and(warp::path::end())
    //     .and(json_body())

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
    Ok(())
}
