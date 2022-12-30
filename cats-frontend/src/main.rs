mod app;
mod routes;
mod index;
mod user;
mod storage;
mod login;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
