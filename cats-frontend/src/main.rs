mod app;
mod routes;
mod index;
mod user;
mod storage;
mod login;
mod api;
mod utils;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
