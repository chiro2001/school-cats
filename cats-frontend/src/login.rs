#![allow(non_snake_case)]

use web_sys::{console, HtmlInputElement};
use yew::{Callback, function_component, Html, html, NodeRef, use_effect_with_deps, use_state};
use yew_router::prelude::*;
use gloo_net::http::Method;
use cats_api::{Empty, Response};
use cats_api::jwt::TokenDB;
use cats_api::user::{LoginPost, LoginResponse, RegisterPost, User};
use crate::api::{API, fetch, fetch_refresh, save_token};
use crate::routes::Route;

#[function_component]
pub fn LoginPage() -> Html {
    let username = NodeRef::default();
    let password = NodeRef::default();
    let onclick = {
        let username = username.clone();
        let passwd = password.clone();
        Callback::from(move |_| {
            let username: String = username.cast::<HtmlInputElement>().unwrap().value().into();
            let passwd: String = passwd.cast::<HtmlInputElement>().unwrap().value().into();
            console::log_2(&"login username:".into(), &username.clone().into());
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<LoginResponse> = fetch(Method::POST, format!("{}/login", API).as_str(),
                                                       LoginPost { username, passwd })
                    .await.unwrap_or(Response::error("error", LoginResponse::default()));
                console::log_1(&format!("{:?}", r).into());
                save_token(&r.data.token).unwrap();
                save_token(&r.data.refresh_token).unwrap();
            });
        })
    };
    let test_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let r: Response<User> = fetch(Method::GET, format!("{}/user", API).as_str(),
                                          Empty::default())
                .await.unwrap_or(Response::error("error", User::default()));
            console::log_1(&format!("{:?}", r).into());
        });
    });
    let refresh_click = Callback::from(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let r: Response<TokenDB> = fetch_refresh(Method::GET, format!("{}/refresh", API).as_str(),
                                                     Empty::default(), true)
                .await.unwrap_or(Response::error("error", TokenDB::default()));
            console::log_1(&format!("{:?}", r).into());
        });
    });
    html! {
        <>
        <h2>{ "登录" }</h2>
        <p><span>{ "用户名" }</span><input ref={username}/></p>
        <p><span>{ "密码" }</span><input type="password" ref={password}/></p>
        <button {onclick}>{ "登录" }</button>
        <Link<Route> to={Route::Register}>{ "注册" }</Link<Route>>
        <button onclick={test_click}>{ "get user" }</button>
        <button onclick={refresh_click}>{ "refresh" }</button>
        </>
    }
}

#[function_component]
pub fn RegisterPage() -> Html {
    let username = NodeRef::default();
    let usernick = NodeRef::default();
    let password = NodeRef::default();
    let motto = NodeRef::default();
    let register_done = use_state(|| false);
    let onclick = {
        let username = username.clone();
        let passwd = password.clone();
        let usernick = usernick.clone();
        let motto = motto.clone();
        let register_done = register_done.clone();
        Callback::from(move |_| {
            let username: String = username.cast::<HtmlInputElement>().unwrap().value().into();
            let passwd: String = passwd.cast::<HtmlInputElement>().unwrap().value().into();
            let usernick: String = usernick.cast::<HtmlInputElement>().unwrap().value().into();
            let motto: String = motto.cast::<HtmlInputElement>().unwrap().value().into();
            let register_done = register_done.clone();
            console::log_2(&"register username:".into(), &username.clone().into());
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<Empty> = fetch(Method::POST, format!("{}/register", API).as_str(),
                                               RegisterPost {
                                                   user: User {
                                                       username,
                                                       uid: 0,
                                                       head: 1,
                                                       usernick,
                                                       motto,
                                                   },
                                                   passwd,
                                               })
                    .await.unwrap_or(Response::err());
                console::log_1(&format!("{:?}", r).into());
                match r.code {
                    200 => register_done.set(true),
                    _ => {}
                };
            });
        })
    };
    match &*register_done {
        true => {
            html! { <Redirect<Route> to={Route::Login} /> }
        }
        false => {
            html! {
            <>
            <h2>{ "注册" }</h2>
            <p><span>{ "用户名" }</span><input ref={username}/></p>
            <p><span>{ "密码" }</span><input type="password" ref={password}/></p>
            <p><span>{ "昵称" }</span><input ref={usernick}/></p>
            <p><span>{ "自我介绍" }</span><input ref={motto}/></p>
            <button {onclick}>{ "注册" }</button>
            </>
            }
        }
    }
}