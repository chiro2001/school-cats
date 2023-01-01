use std::error::Error;
use std::fmt::{Display, Formatter};
use web_sys::console;
use cats_api::user::User;
use crate::storage::{load_string, save_string, storage};
use anyhow::Result;
use gloo_net::http::Method;
use cats_api::{Empty, Response};
use cats_api::jwt::TokenDB;
use crate::api::{API, fetch, fetch_refresh, save_refresh_token, save_token};
use crate::user::TokenError::RefreshTokenInvalid;

#[derive(Debug)]
pub enum TokenError {
    RefreshTokenInvalid,
    // TokenInvalid,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TokenError {}

async fn fetch_refreshed_token(refresh_token: &str) -> Result<TokenDB, TokenError> {
    save_refresh_token(refresh_token).map_err(|_| RefreshTokenInvalid)?;
    let r: Response<TokenDB> = fetch_refresh(Method::GET, &format!("{}/refresh", API), Empty::default(), true)
        .await.map_err(|_| RefreshTokenInvalid)?;
    match r.code {
        200 => Ok(r.data),
        _ => Err(RefreshTokenInvalid)
    }
}

pub async fn load_token() -> Result<String, TokenError> {
    let refresh_token = load_string("refresh_token").map_err(|_| RefreshTokenInvalid)?;
    if refresh_token == "invalid" || refresh_token.is_empty() { return Err(RefreshTokenInvalid); }
    let token = match load_string("token") {
        Ok(t) if t == "invalid" || t.is_empty() => fetch_refreshed_token(&refresh_token).await?.token,
        Ok(t) => t,
        Err(_) => fetch_refreshed_token(&refresh_token).await?.token
    };
    save_token(&token).map_err(|_| RefreshTokenInvalid)?;
    Ok(token)
}

pub fn save_user(u: &User) -> Result<()> {
    save_string("user", &serde_json::to_string(u)?)
}

pub async fn fetch_user() -> Result<User> {
    let r: Response<User> = fetch(Method::GET, &format!("{}/user", API), Empty::default()).await?;
    Ok(r.data)
}

pub async fn load_user() -> Option<User> {
    console::log_1(&"loading user...".into());
    let _token = match load_token().await {
        Ok(token) => token,
        Err(_) => {
            console::log_1(&"no token".into());
            return None;
        }
    };
    let s = storage();
    let u = match s.get_item("user") {
        Ok(v) => match v {
            Some(v) if v.is_empty() => None,
            Some(v) => {
                match serde_json::from_str::<User>(&v) {
                    Ok(u) => Some(u),
                    Err(_) => None
                }
            }
            None => None
        },
        Err(e) => panic!("get localStorage error! {:?}", e)
    };
    let u = match u {
        Some(u) => Some(u),
        None => match fetch_user().await {
            Ok(u) => Some(u),
            Err(_) => None
        }
    };
    if u.is_some() {
        save_user(u.as_ref().unwrap()).unwrap();
    }
    u
}