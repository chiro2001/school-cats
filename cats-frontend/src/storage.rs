use web_sys::Storage;

pub fn storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}