#![allow(non_snake_case)]

use std::ops::Deref;
use yew::prelude::*;
use chrono::{DateTime, Local};
use gloo::console::console;
use gloo_net::http::Method;
use yew::{Callback, html, Html, NodeRef, use_effect_with_deps, use_state, UseStateHandle};
use cats_api::cats::{BreedDB, BreedPost, CatDB, CatDisp};
use cats_api::utils::{chrono2sys, time_fmt};
use crate::api::{API, fetch};
use crate::utils::{node_str, reload};
use anyhow::Result;
use web_sys::console;
use cats_api::{Empty, Response};
use crate::cat::cat_render;

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
            let time = format!("{}:00 +0800", node_str(&self.found_time)).to_string();
            console::log_1(&time.as_str().into());
            let datetime = DateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S %z")?.with_timezone(&Local);
            console::log_1(&format!("datetime: {}", datetime.to_rfc2822()).into());
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
                    Ok(data) => {
                        fetch(Method::POST, format!("{}/cat", API).as_str(), data)
                            .await.unwrap_or(Response::default_error(0));
                    }
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
        <Manager/>
        </>
    }
}

#[function_component]
pub fn Manager() -> Html {
    let cats: UseStateHandle<Vec<CatDB>> = use_state(|| vec![]);
    {
        let cats = cats.clone();
        use_effect_with_deps(move |_| {
            let cats = cats.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let r: Vec<CatDB> = fetch(Method::GET, format!("{}/cat", API).as_str(), Empty::default())
                    .await.unwrap_or(Response::default_error(vec![])).data;
                cats.set(r);
            });
        }, ());
    };
    html! {
        <>
        { for cats.iter().map(cat_render) }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct CatInfoProps {
    pub id: u32,
}

#[function_component]
pub fn CatInfoPage(props: &CatInfoProps) -> Html {
    let cat: UseStateHandle<CatDisp> = use_state(|| CatDisp::default());
    let id = props.id;
    {
        let cat = cat.clone();
        use_effect_with_deps(move |_| {
            let cat = cat.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let r = fetch(Method::GET, format!("{}/cat/{}", API, id).as_str(), Empty::default())
                    .await.unwrap_or(Response::default_error(CatDisp::default())).data;
                cat.set(r);
            });
        }, ());
    };
    if cat.deref().catId == 0 {
        html! {
            <h3>{"loading"}</h3>
        }
    } else {
        let c = cat.deref();
        html! {
            <>
            <h2>{"猫猫信息"}</h2>
            <p>{"猫猫id: "}{c.catId.to_string()}</p>
            <p>{"猫猫名字: "}{c.name.to_string()}</p>
            <p>{"猫猫品种: "}{c.breed.breedName.to_string()}{" "}{c.breed.breedDesc.to_string()}</p>
            <p>{"猫猫发现时间: "}{time_fmt(DateTime::from(c.foundTime))}</p>
            <p>{"猫猫是否在校: "}{c.atSchool.to_string()}</p>
            <p>{"猫猫近况: "}{c.whereabouts.to_string()}</p>
            <p>{"猫猫健康状况: "}{c.health.to_string()}</p>
            </>
        }
    }
}