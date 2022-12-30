#![allow(non_snake_case)]

use web_sys::{console, HtmlInputElement};
use yew::{Callback, function_component, Html, html, NodeRef, use_effect_with_deps};
use gloo_net::http::{Method, Request};
use serde::Deserialize;
use cats_api::Hello;
use crate::api::{API, fetch};

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component]
pub fn LoginPage() -> Html {
    let username = NodeRef::default();
    let password = NodeRef::default();
    let onclick = {
        let username = username.clone();
        let password = password.clone();
        // use_effect_with_deps(move |_| {
        //     wasm_bindgen_futures::spawn_local(async move {});
        // }, ());
        Callback::from(move |_| {
            let username: String = username.cast::<HtmlInputElement>().unwrap().value().into();
            let password: String = password.cast::<HtmlInputElement>().unwrap().value().into();
            console::log_2(&"login/register username:".into(), &username.into());
            wasm_bindgen_futures::spawn_local(async move {
                let r: Hello = fetch(Method::GET, format!("{}/hello/test", API).as_str())
                    .await.unwrap_or(Hello::default());
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