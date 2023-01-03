#![allow(non_snake_case)]

use chrono::{DateTime, Local};
use gloo_net::http::Method;
use yew::prelude::*;
use cats_api::{Empty, Response};
use cats_api::cats::{CatDB, FeedingDB};
use cats_api::places::PlaceDB;
use cats_api::user::User;
use cats_api::utils::{chrono2sys, time_fmt};
use crate::api::{API, fetch};
use crate::user::load_user_local;
use crate::utils::node_str;

#[function_component]
pub fn CatToFeed() -> Html {
    html! {

    }
}

#[function_component]
pub fn CatFeedingRegister() -> Html {
    let places: UseStateHandle<Vec<PlaceDB>> = use_state(|| vec![]);
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
    #[derive(Default, Clone)]
    struct FeedingInput {
        pub time: NodeRef,
        pub food: NodeRef,
        pub amount: NodeRef,
        pub cat: NodeRef,
        pub place: NodeRef,
    }
    let input = FeedingInput::default();
    let onclick = {
        let input = input.clone();
        Callback::from(move |_| {
            let input = input.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user = load_user_local().await.unwrap_or(User::default());
                let time = format!("{}:00 +0800", node_str(&input.time)).to_string();
                let datetime = DateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S %z").unwrap().with_timezone(&Local);
                let time = chrono2sys(datetime.naive_utc());
                let (food, amount) = (node_str(&input.food), node_str(&input.amount));
                let id_cat = node_str(&input.cat);
                let id_place = node_str(&input.place);
                let p = FeedingDB {
                    catId: id_cat.parse::<u32>().unwrap_or(0),
                    userId: user.uid,
                    placeId: id_place.parse::<u32>().unwrap_or(0),
                    feedTime: time,
                    feedFood: food,
                    feedAmount: amount,
                };
                fetch(Method::POST, format!("{}/feeding", API).as_str(), p)
                    .await.unwrap_or(Response::default_error(Empty::default()));
            })
        })
    };
    html! {
        <>
        <span>{"时间: "}<input ref={input.time} type="datetime-local" value={time_fmt(chrono::Local::now())}/></span><br/>
        <span>{"食物: "}<input ref={input.food}/></span><br/>
        <span>{"投喂量: "}<input ref={input.amount}/></span><br/>
        <span>{"猫猫: "}<select ref={input.cat}>
        { for cats.iter().map(|c| html! { <option value={c.catId.to_string()}>{c.name.to_string()}</option> })}
        </select></span><br/>
        <span>{"地点: "}<select ref={input.place}>
        { for places.iter().map(|c| html! { <option value={c.id.to_string()}>{c.details.to_string()}</option> })}
        </select></span><br/>
        <button {onclick}>{"提交登记"}</button>
        </>
    }
}