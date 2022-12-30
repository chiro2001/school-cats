#![allow(non_snake_case)]

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::console;

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
    // let text = use_state(|| "".to_string());
    let images: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    // let oninput = {
    //     let text = text.clone();
    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlTextAreaElement = e.target_unchecked_into();
    //         text.set(input.value());
    //     })
    // };
    let textarea = NodeRef::default();
    let onclick = {
        // let mut images = images.clone();
        let textarea = textarea.clone();
        Callback::from(move |_| {
            console::log_2(&"text:".into(), &textarea.cast::<HtmlTextAreaElement>().unwrap().value().into());
        //     images.push("https://yew.rs/img/logo.png".to_string());
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