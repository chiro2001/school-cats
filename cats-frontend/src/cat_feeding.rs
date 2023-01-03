#![allow(non_snake_case)]

use chrono::{DateTime, Local};
use gloo_net::http::Method;
use yew::prelude::*;
use yew_router::prelude::*;
use cats_api::{Empty, Response};
use cats_api::cats::{CatDB, FeedingDB, FeedingInfo};
use cats_api::places::PlaceDB;
use cats_api::user::User;
use cats_api::utils::{chrono2sys, time_fmt};
use crate::api::{API, fetch};
use crate::cat::cat_disp_render;
use crate::routes::Route;
use crate::user::load_user_local;
use crate::utils::{node_str, reload};

#[function_component]
pub fn CatToFeed() -> Html {
    let feedings: UseStateHandle<Vec<FeedingInfo>> = use_state(|| vec![]);
    {
        let feedings = feedings.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let list: Vec<FeedingInfo> = fetch(
                    Method::GET, format!("{}/to_feed", API).as_str(),
                    Empty::default()).await.unwrap_or(Response::default_error(vec![])).data;
                feedings.set(list);
            });
        }, ());
    };
    let render: fn(&FeedingInfo) -> Html = |f| {
        let datetime: DateTime<Local> = f.last.feedTime.into();
        let time = time_fmt(datetime);
        html! {
            <ul>
            <span>{cat_disp_render(&f.cat)}{"上次由"}
            <Link<Route> to={Route::UserInfo{id: f.user.uid}}>{ format!("[{}]{}", f.user.uid,
                if !f.user.usernick.is_empty() { f.user.usernick.as_str() } else { f.user.username.as_str() }) }
            </Link<Route>>
            {"于"}{time.to_string()}{"投喂"}
            </span>
            </ul>
        }
    };
    html! {
        <>
        <h2>{ "猫猫待哺" }</h2>
        { for feedings.iter().map(render) }
        </>
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
                reload();
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