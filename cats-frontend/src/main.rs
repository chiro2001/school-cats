mod app;
mod routes;
mod index;
mod user;
mod storage;
mod login;
mod api;
mod utils;
mod cat_info;
mod cat_post;
mod cat;
mod cat_feeding;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
