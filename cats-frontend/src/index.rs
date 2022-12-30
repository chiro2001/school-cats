#![allow(non_snake_case)]

use yew::prelude::*;

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

#[function_component]
fn Posts() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {
    <>
        <h2>{ "猫猫贴" }</h2>
        <h4>{ "新的猫猫贴" }</h4>
        <button {onclick}>{ "Inc" }</button>
        <span>
            <b>{ "value: " }</b>
            { *counter }
        </span>
    </>
    }
}

pub fn index() -> Html {
    html! {
        <>
            <h1>{ "主页" }</h1>
            <CatsMap/>
            <CatsFeedings/>
            <Posts/>
        </>
    }
}