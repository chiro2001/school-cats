#![allow(non_snake_case)]

use std::ops::Deref;
use gloo_net::http::Method;
use yew::{html, Html, use_effect_with_deps, use_state, UseStateHandle};
use cats_api::cats::{CatDB, CatPlacesResponse};
use crate::api::{API, fetch};
use yew::prelude::*;
use cats_api::Empty;

#[function_component]
pub fn CatsMap() -> Html {
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
        if *loading {
            <p>{"loading"}</p>
        } else {
            <ul>
            { for places.deref().iter().map(render) }
            </ul>
        }
        </>
    }
}

#[function_component]
pub fn CatsFeedings() -> Html {
    html! {
        <h2>{ "猫猫待哺" }</h2>
    }
}

pub fn cat_render(c: &CatDB) -> Html {
    html! {
        <>
        <span>{&c.name}{" "}</span>
        </>
    }
}
