#![allow(non_snake_case)]

use web_sys::{console, HtmlInputElement};
use yew::{Callback, function_component, Html, html, NodeRef, use_effect_with_deps};
use gloo_net::http::{Method, Request};
use serde::Deserialize;
use cats_api::{Empty, Hello};
use cats_api::user::{LoginPost, LoginResponse};
use crate::api::{API, fetch};

#[function_component]
pub fn LoginPage() -> Html {
    let username = NodeRef::default();
    let password = NodeRef::default();
    let onclick = {
        let username = username.clone();
        let password = password.clone();
        Callback::from(move |_| {
            let username: String = username.cast::<HtmlInputElement>().unwrap().value().into();
            let password: String = password.cast::<HtmlInputElement>().unwrap().value().into();
            console::log_2(&"login/register username:".into(), &username.clone().into());
            wasm_bindgen_futures::spawn_local(async move {
                let r: LoginResponse = fetch(Method::POST, format!("{}/login", API).as_str(),
                                             LoginPost { username, password })
                    .await.unwrap_or(LoginResponse::default());
                console::log_1(&format!("{:?}", r).into());
            });
        })
    };
    html! {
        <>
        <h2>{ "登录" }</h2>
        <p><span>{ "用户名" }</span><input ref={username}/></p>
        <p><span>{ "密码" }</span><input type="password" ref={password}/></p>
        <button {onclick}>{ "登录/注册" }</button>
        </>
    }
}