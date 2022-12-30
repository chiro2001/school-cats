use serde::{Deserialize, Serialize};
use crate::storage::storage;
use anyhow::{anyhow, Result};
use gloo_net::http::{Method, Request};

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

pub fn load_tokens() -> (String, String) {
    let s = storage();
    let default = "invalid";
    let token = load_string("token", default);
    let refresh_token = load_string("refresh_token", default);
    (token, refresh_token)
}

// #[derive(Deserialize, Serialize)]
// pub struct Response<T> {
//     pub code: usize,
//     pub msg: String,
//     pub data: T,
// }

pub async fn fetch<T: for<'de> Deserialize<'de>>(method: Method, url: &str) -> Result<T> {
    let tokens = load_tokens();
    match Request::new(url)
        .method(method)
        // .header("content-type", "application/json")
        // .header("token", &*tokens.0)
        // .header("refresh_token", &*tokens.1)
        .send().await {
        Ok(r) => match r.json().await {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("error: {:?}", e))
        },
        Err(e) => Err(anyhow!("request error: {:?}", e))
    }
}