mod app;
mod routes;
mod index;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
