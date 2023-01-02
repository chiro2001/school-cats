#![allow(non_snake_case)]

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::console;
use crate::routes::Route;
use crate::user::load_user;
use yew_router::prelude::*;

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
    let images: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let textarea = NodeRef::default();
    let onclick = {
        let images = images.clone();
        let textarea = textarea.clone();
        Callback::from(move |_| {
            console::log_2(&"text:".into(), &textarea.cast::<HtmlTextAreaElement>().unwrap().value().into());
            let mut imgs = images.to_vec();
            imgs.push("https://yew.rs/img/logo.png".to_string());
            images.set(imgs);
        })
    };
    html! {
    <>
        <h2>{ "猫猫贴" }</h2>
        <h4>{ "新的猫猫贴" }</h4>
        <div>
            <span>
                <p>{ "正文" }</p>
                // <textarea {oninput}/>
                <textarea ref={textarea}/>
                <p>{ "猫猫图" }</p>
                <ul>
                    { for images.iter().map(|i: &String| html! {<img src={i.clone()}/>}) }
                </ul>
                <button {onclick}>{ "添加图片" }</button>
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