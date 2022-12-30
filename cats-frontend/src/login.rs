#![allow(non_snake_case)]

use web_sys::{console, HtmlInputElement};
use yew::{Callback, function_component, Html, html, NodeRef, use_effect_with_deps};
use gloo_net::http::Request;
use serde::Deserialize;

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
                let v: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
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