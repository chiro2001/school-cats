use yew::prelude::*;
use yew_router::prelude::*;
use crate::cat_info::CatInfoPage;
use crate::index::IndexPage;
use crate::login::{LoginPage, RegisterPage};
use crate::routes::Route;
use crate::user::{UserInfoComponent, UserInfoPage};

fn switch(route: Route) -> Html {
    use Route::*;
    match route {
        Index => { html! { <IndexPage/> } }
        Login => { html! { <LoginPage/> } }
        Register => { html! { <RegisterPage/> } }
        UserInfo { id } => { html! { <UserInfoPage {id}/> } }
        CatInfo { id } => { html! { <CatInfoPage {id}/> } }
        _ => html! {
            <h1>{ "你来到了没有猫猫的可怕地方！" }</h1>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <UserInfoComponent/>
            <BrowserRouter>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </main>
    }
}
