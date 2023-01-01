use serde::{Deserialize, Serialize};
use crate::storage::storage;
use anyhow::{anyhow, Result};
use gloo_net::http::{Method, Request};
use cats_api::Empty;

pub const API: &str = "http://127.0.0.1:3030";

fn load_string(key: &str, default: &str) -> String {
    let s = storage();
    match s.get_item(key) {
        Ok(v) => match v {
            Some(v) => v,
            None => default.to_string()
        },
        Err(e) => panic!("cannot load token! {:?}", e)
    }
}

fn save_string(key: &str, value: &str) {
    let s = storage();
    s.set_item(key, value);
}

pub fn load_tokens() -> (String, String) {
    let default = "invalid";
    let token = load_string("token", default);
    let refresh_token = load_string("refresh_token", default);
    (token, refresh_token)
}

pub fn save_token(token: &str) -> Result<()> {
    save_string("token", token);
    Ok(())
}

pub fn save_refresh_token(refresh_token: &str) -> Result<()> {
    save_string("refresh_token", refresh_token);
    Ok(())
}

// #[derive(Deserialize, Serialize)]
// pub struct Response<T> {
//     pub code: usize,
//     pub msg: String,
//     pub data: T,
// }

pub async fn fetch<B: Serialize, T: for<'de> Deserialize<'de>>(method: Method, url: &str, body: B) -> Result<T> {
    let tokens = load_tokens();
    let pre = Request::new(url)
        .method(method)
        .header("Content-Type", "application/json")
        .header("Token", &*tokens.0);
    let pre = match method {
        Method::GET => pre,
        _ => pre.body(serde_json::to_string(&body)?)
    };
    match pre
        .send().await {
        Ok(r) => match r.json().await {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("error: {:?}", e))
        },
        Err(e) => Err(anyhow!("request error: {:?}", e))
    }
}