#![allow(non_snake_case)]

use std::collections::HashSet;
use std::ops::Deref;
use yew::prelude::*;
use chrono::{DateTime, Local};
use gloo::console::console;
use gloo_net::http::Method;
use yew::{Callback, html, Html, NodeRef, use_effect_with_deps, use_state, UseStateHandle};
use crate::api::{API, fetch};
use crate::utils::{node_str, reload};
use web_sys::{console, HtmlTextAreaElement};
use cats_api::{Empty, Response};
use cats_api::places::{PlaceDB, PlacePost};
use cats_api::posts::{PostDisp, PostsPost};
use crate::cat::cat_render;

#[function_component]
pub fn Posts() -> Html {
    let posts: UseStateHandle<Vec<PostDisp>> = use_state(|| vec![]);
    let images: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let places: UseStateHandle<Vec<PlaceDB>> = use_state(|| vec![]);
    let places_selected: UseStateHandle<Vec<PlaceDB>> = use_state(|| vec![]);
    let textarea = NodeRef::default();
    {
        let places = places.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let list: Vec<PlaceDB> = fetch(
                    Method::GET, format!("{}/place", API).as_str(),
                    Empty::default()).await.unwrap_or(Response::default_error(vec![])).data;
                places.set(list);
            });
        }, ());
    };
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
        let places_selected = places_selected.clone();
        let textarea = textarea.clone();
        Callback::from(move |_| {
            let text: String = textarea.cast::<HtmlTextAreaElement>().unwrap().value();
            let images = images.to_vec();
            let places = places_selected.to_vec().into_iter().map(|i| i.id).collect::<Vec<u32>>();
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<Empty> = fetch(
                    Method::POST, format!("{}/post", API).as_str(),
                    PostsPost { text, images, places })
                    .await.unwrap_or(Response::default_error(Empty::default()));
                console::log_1(&format!("{:?}", r).into());
                reload();
            });
        })
    };
    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<Vec<PostDisp>> = fetch(
                    Method::GET, format!("{}/post", API).as_str(),
                    Empty::default(),
                ).await.unwrap_or(Response::default_error(vec![]));
                posts.set(r.data);
            });
        }, ());
    };
    let post_render: fn(&PostDisp) -> Html = |p| {
        let image_render: fn(&String) -> Html = |s| html! { <img src={s.to_string()}/> };
        html! {
            <div>
            <p>{ format!("user: [{}]{}", p.user.uid,
                if !p.user.usernick.is_empty() { p.user.usernick.as_str() } else { p.user.username.as_str() }) }</p>
            <p>{ "发送时间: " } {{
                let datetime: DateTime<Local> = p.time.into();
                &datetime.format("%m-%d %H:%M").to_string()
                }}</p>
            if !p.text.is_empty() { <p>{ &p.text }</p> }
            <div>
            { for p.images.iter().map(image_render) }
            </div>
            if !p.places.is_empty() {
                <div>
                { "地点: " } { for p.places.iter().map(|s| html! { <span>{s}{" "}</span> }) }
                </div>
            }
            if !p.cats.is_empty() {
                <div>{"猫猫: "} { for p.cats.iter().map(cat_render) }</div>
            }
            </div>
        }
    };
    let place_input = NodeRef::default();
    let place_select = NodeRef::default();
    let add_place = {
        let place_input = place_input.clone();
        Callback::from(move |_| {
            let text: String = node_str(&place_input);
            if text.is_empty() { return; }
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<u32> = fetch(Method::POST, format!("{}/place", API).as_str(), PlacePost { details: text.to_string() })
                    .await.unwrap_or(Response::default_error(0));
                if r.code != 0 {
                    reload();
                }
            });
        })
    };
    let select_place = {
        let places = places.clone();
        let places_selected = places_selected.clone();
        let place_select = place_select.clone();
        Callback::from(move |_| {
            let mut list = places_selected.to_vec();
            let s: HashSet<PlaceDB> = HashSet::from_iter(list.iter().map(|p| p.copy()));
            let id = node_str(&place_select);
            let _ = places.deref().iter().filter(|p| p.id.to_string() == id).map(|p| {
                if !s.contains(p) {
                    console!(format!("using place: {:?}", p));
                    list.push(p.copy());
                }
            }).collect::<Vec<_>>();
            places_selected.set(list);
        })
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
                <div>
                    <span>{ "地点: " }<span>{ for places_selected.iter().map(|p: &PlaceDB| html! {<>{p.details.to_string()}{" "}</>}) }</span></span><br/>
                    <select ref={place_select}>
                        { for places.iter().map(|p: &PlaceDB| html! { <option value={p.id.to_string()}>{p.details.to_string()}</option> })}
                    </select>
                    <button onclick={select_place}>{ "选择地点" }</button>
                    <input ref={place_input}/><button onclick={add_place}>{ "添加新地点" }</button>
                </div>
                <button onclick={push_image}>{ "添加图片" }</button>
                <button onclick={post}>{ "发布猫猫贴" }</button>
            </span>
        </div>
    </>
    }
}