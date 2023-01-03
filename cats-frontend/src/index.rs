#![allow(non_snake_case)]

use web_sys::console;
use crate::cat::CatsMap;
use crate::cat_info::Information;
use crate::cat_post::Posts;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::cat_feeding::CatToFeed;
use crate::routes::Route;
use crate::user::load_user;

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
            <CatToFeed/>
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