mod db;
mod upload;

use warp::{Filter, http};
use cats_api::*;
use crate::db::{Database, db_get_pool};
use anyhow::Result;
use log::{error, info};
use warp::http::Method;
use cats_api::cats::{BreedPost, CatDB, CatDisp, FeedingDB};
use cats_api::jwt::TokenDB;
use cats_api::places::PlacePost;
use cats_api::posts::{CommentPost, PostsPost};
use cats_api::user::{LoginPost, LoginResponse, RegisterPost, User, UserDB};
use crate::upload::upload_image;

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

    let dbc = db.clone();
    let user_uid_get = warp::get()
        .and(warp::path("user"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .map(move |uid| warp::reply::json(&match dbc.user(uid) {
            Ok(u) => Response::ok(u),
            Err(e) => Response::error(&e.to_string(), User::default())
        }));

    let dbc = db.clone();
    let user_get = warp::get()
        .and(warp::path("user"))
        .and(warp::path::end())
        .and(warp::header::<String>("token"))
        .map(move |token: String| warp::reply::json(&match dbc.token_check(&token) {
            Ok(t) => {
                match dbc.user(t.uid) {
                    Ok(u) => Response::ok(u),
                    Err(e) => Response::error(&e.to_string(), User::default())
                }
            }
            Err(e) => Response::error(&e.to_string(), User::default())
        }));

    let dbc = db.clone();
    let login_post = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .map(move |r: LoginPost| warp::reply::json(&match dbc.user_check(&r.username, &r.passwd).map(|uid| {
            let token = dbc.create_token(uid);
            if token.is_err() {
                error!("cannot create token!");
            }
            let refresh = dbc.create_refresh_token(uid);
            if refresh.is_err() {
                error!("cannot create refresh token!");
            }
            [token, refresh].map(|t| t.map(|t| t.0)).into_iter().collect::<Result<Vec<String>>>()
        }) {
            Ok(Ok(r)) => {
                Response::ok(LoginResponse {
                    token: r.get(0).unwrap_or(&"".to_string()).clone(),
                    refresh_token: r.get(1).unwrap_or(&"".to_string()).clone(),
                })
            }
            Err(e) => Response::error(&e.to_string(), LoginResponse::default()),
            Ok(Err(e)) => Response::error(&e.to_string(), LoginResponse::default()),
        }));

    let dbc = db.clone();
    let token_refresh = warp::get()
        .and(warp::path("refresh"))
        .and(warp::header::<String>("refresh-token"))
        .map(move |token: String| warp::reply::json(&match dbc.token_check(&token) {
            Ok(t) => {
                match dbc.create_token(t.uid) {
                    Ok((new_token, exp)) => {
                        Response::ok(TokenDB {
                            token: new_token,
                            exp,
                            uid: t.uid,
                        })
                    }
                    Err(e) => Response::error(&e.to_string(), TokenDB::default())
                }
            }
            Err(e) => Response::error(&e.to_string(), TokenDB::default())
        }));

    let dbc = db.clone();
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(move |r: RegisterPost| warp::reply::json(&match dbc.user_insert(UserDB::from_user(r.user, 0, &r.passwd)) {
            Ok(_uid) => Response::ok(Empty::default()),
            Err(e) => Response::error(&e.to_string(), Empty::default())
        }));

    let dbc = db.clone();
    let post_post = warp::post()
        .and(warp::path("post"))
        .and(warp::header::<String>("token"))
        .and(warp::body::json())
        .map(move |token: String, r: PostsPost| warp::reply::json(&match dbc.token_check(&token) {
            Ok(t) => {
                match dbc.post_insert(t.uid, r) {
                    Ok(_) => Response::ok(Empty::default()),
                    Err(e) => Response::error(&e.to_string(), Empty::default())
                }
            }
            Err(e) => Response::error(&e.to_string(), Empty::default())
        }));

    let dbc = db.clone();
    let post_get = warp::get()
        .and(warp::path("post"))
        .map(move || warp::reply::json(&match dbc.post_list() {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), vec![]),
        }));

    let dbc = db.clone();
    let cat_places_get = warp::get()
        .and(warp::path("cat_places"))
        .map(move || warp::reply::json(&match dbc.cats_places() {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), vec![]),
        }));

    let dbc = db.clone();
    let place_post = warp::post()
        .and(warp::path("place"))
        .and(warp::body::json())
        .map(move |p: PlacePost| warp::reply::json(&match dbc.place_insert(&p.details) {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), 0),
        }));

    let dbc = db.clone();
    let place_get = warp::get()
        .and(warp::path("place"))
        .map(move || warp::reply::json(&match dbc.place_list() {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), vec![]),
        }));

    let dbc = db.clone();
    let cat_post = warp::post()
        .and(warp::path("cat"))
        .and(warp::body::json())
        .map(move |c: CatDB| warp::reply::json(&match dbc.cat_insert(c) {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), 0),
        }));

    let dbc = db.clone();
    let cat_get = warp::get()
        .and(warp::path("cat"))
        .and(warp::path::end())
        .map(move || warp::reply::json(&match dbc.cats() {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), vec![]),
        }));

    let dbc = db.clone();
    let cat_id_get = warp::get()
        .and(warp::path("cat"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .map(move |id| warp::reply::json(&match dbc.cat(id) {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), CatDisp::default()),
        }));

    let dbc = db.clone();
    let breed_get = warp::get()
        .and(warp::path("breed"))
        .map(move || warp::reply::json(&match dbc.breed_list() {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), vec![]),
        }));

    let dbc = db.clone();
    let breed_post = warp::post()
        .and(warp::path("breed"))
        .and(warp::body::json())
        .map(move |b: BreedPost| warp::reply::json(&match dbc.breed_insert_or(b) {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), 0),
        }));

    let dbc = db.clone();
    let feeding_post = warp::post()
        .and(warp::path("feeding"))
        .and(warp::body::json())
        .map(move |f: FeedingDB| warp::reply::json(&match dbc.feeding_insert(f) {
            Ok(()) => Response::ok(Empty::default()),
            Err(e) => Response::error(&e.to_string(), Empty::default())
        }));

    let dbc = db.clone();
    let to_feed_get = warp::get()
        .and(warp::path("to_feed"))
        .map(move || warp::reply::json(&match dbc.to_feed() {
            Ok(p) => Response::ok(p),
            Err(e) => Response::error(&e.to_string(), vec![])
        }));

    let dbc = db.clone();
    let comment_post = warp::post()
        .and(warp::path("comment"))
        .and(warp::header::<String>("token"))
        .and(warp::body::json())
        .map(move |token: String, r: CommentPost| warp::reply::json(&match dbc.token_check(&token) {
            Ok(t) => {
                match dbc.comment_insert(t.uid, r) {
                    Ok(_) => Response::ok(Empty::default()),
                    Err(e) => Response::error(&e.to_string(), Empty::default())
                }
            }
            Err(e) => Response::error(&e.to_string(), Empty::default())
        }));

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(500000000))
        .and_then(upload_image);
    let download_route = warp::path("files")
        .and(warp::fs::dir("./files/"));

    let routes = warp::any().and(
        index
            .or(hello)
            .or(user_get)
            .or(user_uid_get)
            .or(login_post)
            .or(register)
            .or(token_refresh)
            .or(post_post)
            .or(post_get)
            .or(cat_places_get)
            .or(place_post)
            .or(place_get)
            .or(cat_post)
            .or(cat_get)
            .or(cat_id_get)
            .or(breed_get)
            .or(breed_post)
            .or(feeding_post)
            .or(to_feed_get)
            .or(comment_post)
            .or(upload_route)
            .or(download_route)
    ).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
    Ok(())
}
