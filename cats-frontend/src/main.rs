mod app;
mod routes;
mod index;
mod user;
mod storage;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
