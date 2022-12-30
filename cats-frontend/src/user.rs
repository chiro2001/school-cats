use web_sys::{console, Storage};
use cats_api::user::User;
use crate::storage::storage;

async fn fetch_token(username: &str) -> &str {
    ""
}

pub async fn load_user() -> Option<User> {
    console::log_1(&"loading user...".into());
    let local_storage = storage();
    match local_storage.get_item("username") {
        Ok(v) => match v {
            Some(v) => {
                None
            }
            None => {
                // need to login / register to get username, will jump to login page
                Some(User::default())
            }
        },
        Err(e) => panic!("get localStorage error! {:?}", e)
    }
}