use warp::Filter;
use cats_api::*;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("school-cats backend! server running on http://127.0.0.1:{}/", PORT);
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}", name));
    warp::serve(hello).run(([127, 0, 0, 1], PORT)).await
}
