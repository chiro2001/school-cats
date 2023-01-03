#![allow(non_snake_case)]

use std::ops::Deref;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use gloo_net::http::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::console;
use crate::routes::Route;
use crate::user::load_user;
use yew_router::prelude::*;
use cats_api::{Empty, Response};
use cats_api::posts::{PostDisp, PostsPost};
use crate::api::{API, fetch};

#[function_component]
fn CatsMap() -> Html {
    html! {
        <h2>{ "猫猫地图" }</h2>
    }
}

#[function_component]
fn CatsFeedings() -> Html {
    html! {
        <h2>{ "猫猫待哺" }</h2>
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct PlaceItem {
    pub id: u32,
    pub details: String,
}

#[function_component]
fn Posts() -> Html {
    let posts: UseStateHandle<Vec<PostDisp>> = use_state(|| vec![]);
    let images: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let places: UseStateHandle<Vec<PlaceItem>> = use_state(|| vec![]);
    let textarea = NodeRef::default();
    let push_image = {
        let images = images.clone();
        let textarea = textarea.clone();
        Callback::from(move |_| {
            console::log_2(&"text:".into(), &textarea.cast::<HtmlTextAreaElement>().unwrap().value().into());
            let mut imgs = images.to_vec();
            imgs.push("https://yew.rs/img/logo.png".to_string());
            images.set(imgs);
        })
    };
    let post = {
        let images = images.clone();
        let textarea = textarea.clone();
        Callback::from(move |_| {
            let text: String = textarea.cast::<HtmlTextAreaElement>().unwrap().value();
            let images = images.to_vec();
            let places = places.to_vec().into_iter().map(|i| i.id).collect::<Vec<u32>>();
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<Empty> = fetch(
                    Method::POST, format!("{}/post", API).as_str(),
                    PostsPost { text, images, places })
                    .await.unwrap_or(Response::default_error(Empty::default()));
                console::log_1(&format!("{:?}", r).into());
                web_sys::window().unwrap().location().reload().unwrap();
            });
        })
    };
    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<Vec<PostDisp>> = fetch(
                    Method::GET, format!("{}/post", API).as_str(),
                    Empty::default()
                ).await.unwrap_or(Response::default_error(vec![]));
                posts.set(r.data);
            });
        }, ());
    };
    let post_render: fn(&PostDisp) -> Html = |p| {
        let image_render: fn(&String) -> Html = |s| html! { <img src={s.to_string()}/> };
        html! {
            <div>
                <p>{ format!("user: [{}]{}", p.user.uid, p.user.usernick) }</p>
                <p>{ "发送时间: " } {{
                    let datetime: DateTime<Utc> = p.time.into();
                    &format!("{:?}", datetime)
                    }}</p>
                if !p.text.is_empty() { <p>{ &p.text }</p> }
                <div>
                { for p.images.iter().map(image_render) }
                </div>
                if !p.places.is_empty() {
                    <div>
                    { "地点: " } { for p.places.iter().map(|s| html! { <span>{s}</span> }) }
                    </div>
                }
            </div>
        }
    };
    html! {
    <>
        <h2>{ "猫猫贴" }</h2>
        { for posts.deref().iter().map(post_render) }
        <h4>{ "新的猫猫贴" }</h4>
        <div>
            <span>
                <p>{ "正文" }</p>
                <textarea ref={textarea}/>
                <p>{ "猫猫图" }</p>
                <ul>
                    { for images.iter().map(|i: &String| html! {<img src={i.clone()}/>}) }
                </ul>
                <button onclick={push_image}>{ "添加图片" }</button>
                <button onclick={post}>{ "发布猫猫贴" }</button>
            </span>
        </div>
    </>
    }
}

#[function_component]
pub fn IndexPage() -> Html {
    let user = use_state(|| None);
    let loading = use_state(|| true);
    {
        let user = user.clone();
        let loading = loading.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_user = load_user().await;
                console::log_1(&format!("fetched_user: {:?}", fetched_user).into());
                user.set(fetched_user);
                loading.set(false);
            });
        }, ());
    };
    let common = html! {
        <>
            <h1>{ "主页" }</h1>
            <p>{ match &*user {
                None => "None".to_string(),
                Some(u) => format!("{:?}", u).to_string(),
            } }</p>
            <CatsMap/>
            <CatsFeedings/>
            <Posts/>
        </>
    };
    console::log_1(&format!("effect user: {:?}", *user).into());
    let login = html! { <Redirect<Route> to={Route::Login}/> };
    if *loading {
        html! {
            <>
            <p>{ "loading" }</p>
            </>
        }
    } else {
        match &*user {
            None => login,
            _ => common
        }
    }
}