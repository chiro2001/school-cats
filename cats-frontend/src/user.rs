// use web_sys::{console, local_storage};
use web_sys::console;
use cats_api::user::User;

pub fn load_user() -> Option<User> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    console::log_1(&"testing".into());
    None
}