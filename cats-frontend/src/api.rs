use serde::{Deserialize, Serialize};
use crate::storage::{load_string_or, save_string};
use anyhow::{anyhow, Result};
use gloo_net::http::{Method, Request};
use cats_api::Response;
use crate::user::load_token;

pub const API: &str = "http://127.0.0.1:3030";

pub fn load_tokens() -> (String, String) {
    let default = "invalid";
    let token = load_string_or("token", default);
    let refresh_token = load_string_or("refresh_token", default);
    (token, refresh_token)
}

pub fn save_token(token: &str) -> Result<()> {
    save_string("token", token)?;
    Ok(())
}

pub fn save_refresh_token(refresh_token: &str) -> Result<()> {
    save_string("refresh_token", refresh_token)?;
    Ok(())
}

pub async fn fetch_refresh<B: Serialize, T: for<'de> Deserialize<'de>>(method: Method, url: &str, body: B, refresh: bool, ignore_tokens: bool) -> Result<Response<T>> {
    if !ignore_tokens {
        let _ = load_token().await?;
    }
    let tokens = load_tokens();
    let pre = Request::new(url)
        .method(method)
        .header("Content-Type", "application/json")
        .header("Token", &*tokens.0);
    let pre = if refresh {
        pre.header("Refresh-Token", &*tokens.1)
    } else {
        pre
    };
    let pre = match method {
        Method::GET => pre,
        _ => pre.body(serde_json::to_string(&body)?)
    };
    match pre
        .send().await {
        Ok(r) => match r.json::<Response<T>>().await {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("error: {:?}", e))
        },
        Err(e) => Err(anyhow!("request error: {:?}", e))
    }
}

pub async fn fetch<B: Serialize + Clone, T: for<'de> Deserialize<'de>>(method: Method, url: &str, body: B) -> Result<Response<T>> {
    let v = fetch_refresh(method, url, body.clone(), false, false).await;
    match v {
        Ok(v) => {
            match v.code {
                200 => Ok(v),
                _ => {
                    match v.msg.as_str() {
                        "token exp failed" => {
                            save_token("invalid").unwrap();
                            fetch_refresh(method, url, body, false, false).await
                        }
                        s => Err(anyhow!(s.to_string()))
                    }
                }
            }
        }
        Err(e) => Err(e)
    }
}