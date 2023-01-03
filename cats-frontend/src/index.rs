#![allow(non_snake_case)]

use std::ops::Deref;
use chrono::{DateTime, Local, Utc};
use gloo_net::http::Method;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::console;
use crate::routes::Route;
use crate::user::load_user;
use yew_router::prelude::*;
use cats_api::{Empty, Response};
use cats_api::cats::{BreedDB, BreedPost, CatDB, CatPlacesResponse};
use cats_api::places::PlacePost;
use cats_api::posts::{PostDisp, PostsPost};
use cats_api::utils::{chrono2sys, time_fmt};
use crate::api::{API, fetch};
use anyhow::Result;
use gloo::console::console;
use crate::utils::{node_str, reload};

#[function_component]
fn CatsMap() -> Html {
    let places: UseStateHandle<Vec<CatPlacesResponse>> = use_state(|| vec![]);
    let loading = use_state(|| true);
    {
        let places = places.clone();
        let loading = loading.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let p = fetch(Method::GET, format!("{}/cat_places", API).as_str(), Empty::default())
                    .await.unwrap_or(vec![]);
                places.set(p);
                loading.set(false);
            });
        }, ());
    };
    let render: fn(&CatPlacesResponse) -> Html = |c| {
        html! {
            <span>
            <span>{ format!("[{}] {}: ", c.cat.catId, c.cat.name) }</span>
            <span>{ for c.places.iter() }</span>
            </span>
        }
    };
    html! {
        <>
        <h2>{ "猫猫地图" }</h2>
        <ul>
        { for places.deref().iter().map(render) }
        </ul>
        </>
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
    let place_input = NodeRef::default();
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
        let places = places.clone();
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
            </div>
        }
    };
    let add_place = {
        let places = places.clone();
        let place_input = place_input.clone();
        Callback::from(move |_| {
            let places = places.clone();
            let text: String = place_input.cast::<HtmlTextAreaElement>().unwrap().value();
            if text.is_empty() { return; }
            wasm_bindgen_futures::spawn_local(async move {
                let r: Response<u32> = fetch(Method::POST, format!("{}/place", API).as_str(), PlacePost { details: text.to_string() })
                    .await.unwrap_or(Response::default_error(0));
                let mut list = places.to_vec();
                if r.code != 0 {
                    list.push(PlaceItem { id: r.data, details: text });
                    places.set(list);
                }
            });
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
                    <p>{ "地点: " }<span>{ for places.iter().map(|p: &PlaceItem| html! {<>{p.details.to_string()}{" "}</>}) }</span></p>
                    <input ref={place_input}/>
                    <button onclick={add_place}>{ "添加地点" }</button>
                </div>
                <button onclick={push_image}>{ "添加图片" }</button>
                <button onclick={post}>{ "发布猫猫贴" }</button>
            </span>
        </div>
    </>
    }
}

#[function_component]
pub fn Information() -> Html {
    #[derive(Default, Clone)]
    struct CatInput {
        pub name: NodeRef,
        pub breed: NodeRef,
        pub found_time: NodeRef,
        pub source: NodeRef,
    }
    #[derive(Default, Clone)]
    struct BreedInput {
        pub name: NodeRef,
        pub desc: NodeRef,
    }
    impl CatInput {
        pub async fn data(&self) -> Result<CatDB> {
            let datetime = DateTime::parse_from_rfc3339(&node_str(&self.found_time))?.with_timezone(&Local);
            let foundTime = chrono2sys(datetime.naive_utc());
            // post breed
            let breedId = fetch(
                Method::GET, format!("{}/breed", API).as_str(),
                BreedPost { name: node_str(&self.breed), desc: "".to_string() })
                .await.unwrap_or(Response::default_error(1_u32)).data;
            let cat = CatDB {
                catId: 0,
                breedId,
                name: node_str(&self.name),
                foundTime,
                source: node_str(&self.source),
                atSchool: true,
                whereabouts: "".to_string(),
                health: "".to_string(),
            };
            Ok(cat)
        }
    }
    impl BreedInput {
        pub async fn post(&self) -> Result<u32> {
            let name = node_str(&self.name);
            let desc = node_str(&self.desc);
            if name.is_empty() { return Ok(0); }
            Ok(fetch(
                Method::POST, format!("{}/breed", API).as_str(),
                BreedPost { name, desc })
                .await.unwrap_or(Response::default_error(1_u32))
                .data)
        }
    }
    let input = CatInput::default();
    let onclick_add = {
        let input = input.clone();
        Callback::from(move |_| {
            let input = input.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match input.data().await {
                    Ok(data) => {}
                    Err(e) => { console!(format!("{:?}", e)); }
                };
            })
        })
    };
    let input_breed = BreedInput::default();
    let onclick_breed = {
        let input_breed = input_breed.clone();
        Callback::from(move |_| {
            let input_breed = input_breed.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match input_breed.post().await {
                    Ok(id) if id > 1 => {
                        reload();
                    }
                    _ => {}
                };
            });
        })
    };
    let breeds: UseStateHandle<Vec<BreedDB>> = use_state(|| vec![]);
    {
        let breeds = breeds.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let list: Vec<BreedDB> = fetch(
                    Method::GET, format!("{}/breed", API).as_str(),
                    Empty::default()).await.unwrap_or(Response::default_error(vec![])).data;
                breeds.set(list);
            });
        }, ());
    };
    let breed_desc = use_state(|| "".to_string());
    let breed_change = {
        let input = input.breed.clone();
        let breeds = breeds.clone();
        let breed_desc = breed_desc.clone();
        Callback::from(move |_| {
            let v = node_str(&input);
            console!(format!("{:?}", v));
            let _ = breeds.deref().iter().filter(|b| b.breedId.to_string() == v).map(|b| {
                breed_desc.set(b.breedDesc.to_string());
            }).collect::<Vec<()>>();
        })
    };
    html! {
        <>
        <h2>{ "登记信息" }</h2>
        <h3>{ "添加品种" }</h3>
        <>
        <span>{ "名称" }<input ref={input_breed.name}/></span><br/>
        <span>{ "描述" }<input ref={input_breed.desc}/></span><br/>
        <button onclick={onclick_breed}>{ "添加" }</button>
        </>
        <h3>{ "添加猫猫" }</h3>
        <>
        <span>{ "名字" }<input ref={input.name}/></span><br/>
        <span>{ "品种" }<select ref={input.breed} onchange={breed_change}>
        { for breeds.iter().map(|breed| html!{ <option value={breed.breedId.to_string()}>{breed.breedName.to_string()}</option> })}
        </select></span><span>{ &*breed_desc }</span><br/>
        <span>{ "发现时间" }<input ref={input.found_time} type="datetime-local" value={time_fmt(chrono::Local::now())}/></span><br/>
        <span>{ "来源" }<input ref={input.source}/></span><br/>
        <button onclick={onclick_add}>{ "添加" }</button>
        </>
        <h3>{ "管理猫猫" }</h3>
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
            <CatsMap/>
            <CatsFeedings/>
            <Posts/>
            <Information/>
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