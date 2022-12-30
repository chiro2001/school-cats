mod app;
mod routes;
mod index;
mod user;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
