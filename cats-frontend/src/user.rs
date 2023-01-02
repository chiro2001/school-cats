#![allow(non_snake_case)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use web_sys::console;
use cats_api::user::User;
use crate::storage::{load_string, save_string, storage};
use anyhow::{anyhow, Result};
use gloo_net::http::Method;
use yew::{Html, function_component, html, use_state, use_effect_with_deps, Callback};
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

pub async fn load_user_local() -> Result<User> {
    let missing = Err(anyhow!("can not load user from local storage"));
    match storage().get_item("user") {
        Ok(v) => match v {
            Some(v) if v.is_empty() => missing,
            Some(v) => {
                match serde_json::from_str::<User>(&v) {
                    Ok(u) => Ok(u),
                    Err(_) => missing
                }
            }
            None => missing
        },
        Err(e) => panic!("get localStorage error! {:?}", e)
    }
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
    let u = load_user_local().await;
    let u = match u {
        Ok(u) => Some(u),
        Err(_) => match fetch_user().await {
            Ok(u) => Some(u),
            Err(_) => None
        }
    };
    if u.is_some() {
        save_user(u.as_ref().unwrap()).unwrap();
    }
    u
}

#[function_component]
pub fn UserInfoComponent() -> Html {
    let user = use_state(|| None);
    {
        let user = user.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let user_local = load_user_local().await;
                match user_local {
                    Ok(u) => user.set(Some(u)),
                    Err(_) => user.set(None)
                }
            });
        }, ());
    };
    let logout = Callback::from(move |_| {
        let f: fn() -> Result<()> = move || {
            save_token("invalid")?;
            save_refresh_token("invalid")?;
            save_string("user", "")?;
            web_sys::window().unwrap().location().reload().unwrap();
            Ok(())
        };
        match f() {
            Ok(()) => {}
            Err(e) => console::error_1(&e.to_string().into()),
        };
    });
    match &*user {
        Some(user) => html! {
            <div>
            <div>{ format!("登录为: [{}]{}", user.uid, user.username) }</div>
            <button onclick={logout}>{ "退出登录" }</button>
            </div>
        },
        None => html! {
            <div>{ "未登录" }</div>
        }
    }
}