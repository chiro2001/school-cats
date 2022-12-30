use serde::{Deserialize, Serialize};
use crate::storage::storage;
use anyhow::{anyhow, Result};
use gloo_net::http::{Method, Request};

const API: &str = "http://localhost:8080/";

fn load_string(key: &str) -> String {
    let s = storage();
    match s.get_item(key) {
        Ok(v) => match v {
            Some(v) => v,
            None => "".to_string()
        },
        Err(e) => panic!("cannot load token! {:?}", e)
    }
}

pub fn load_tokens() -> (String, String) {
    let s = storage();
    let token = load_string("token");
    let refresh_token = load_string("refresh_token");
    (token, refresh_token)
}

// #[derive(Deserialize, Serialize)]
// pub struct Response<T> {
//     pub code: usize,
//     pub msg: String,
//     pub data: T,
// }

pub async fn fetch<T>(method: Method, url: &str) -> Result<T> {
    let tokens = load_tokens();
    match Request::new(url)
        .method(method)
        .header("token", &*tokens.0)
        .header("refresh_token", &*tokens.1)
        .send().await {
        Ok(r) => r.json(),
        Err(e) => Err(anyhow!("request error: {:?}", e))
    }
}