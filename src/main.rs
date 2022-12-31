mod db;

use warp::{Filter, http};
use cats_api::*;
use crate::db::{Database, db_get_pool};
use anyhow::Result;
use log::info;
use warp::http::Method;
use cats_api::user::{LoginPost, LoginResponse, RegisterPost, User};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    info!("school-cats backend!");
    let pool = db_get_pool().await?;
    let db = Database::new(pool);
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
        .map(|name: String| warp::reply::json(
            &Response::ok(Hello { msg: name })
        ));

    let user_uid_get = warp::get()
        .and(warp::path("user"))
        .and(warp::path::param::<u32>())
        .map(|_uid| warp::reply::json(&Response::ok(User::default())));

    let dbc = db.clone();
    let user_get = warp::get()
        .and(warp::path("user"))
        .and(warp::header::<String>("token"))
        .map(move |token: String| warp::reply::json(&match dbc.token_check(&token) {
            Ok(t) => {
                Response::ok(User::default())
            }
            Err(_) => Response::ok(User::default())
        }));

    let dbc = db.clone();
    let user_get2 = warp::get()
        .and(warp::path("user"))
        .and(warp::header::<String>("token"))
        .map(move |token: String| warp::reply::json(&match dbc.token_check(&token) {
            Ok(t) => {
                Response::ok(User::default())
            }
            Err(_) => Response::ok(User::default())
        }));

    let login_post = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .map(|r: LoginPost| warp::reply::json(&Response::ok(LoginResponse { token: r.username, refresh_token: "".to_string() })));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(|_r: RegisterPost| warp::reply::json(&Response::ok(Empty::default())));

    let routes = warp::any().and(
        index
            .or(hello)
            .or(user_get)
            .or(user_get2)
            .or(user_uid_get)
            .or(login_post)
            .or(register)
    ).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
    Ok(())
}
