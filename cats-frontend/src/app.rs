use yew::prelude::*;
use yew_router::prelude::*;
use crate::index::index;
use crate::routes::Route;

fn switch(route: Route) -> Html {
    use Route::*;
    match route {
        Index => {
            index()
        }
        _ => html! {
            <h1>{ "你来到了没有猫猫的可怕地方！" }</h1>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}
