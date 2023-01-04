use web_sys::{console, Storage};
use anyhow::{Result, anyhow};

pub fn storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

pub fn load_string(key: &str) -> Result<String> {
    let s = storage();
    match s.get_item(key) {
        Ok(s) => match s {
            Some(s) => Ok(s),
            None => Err(anyhow!("not set"))
        },
        Err(e) => Err(anyhow!("{:?}", e))
    }
}

pub fn load_string_or(key: &str, default: &str) -> String {
    match load_string(key) {
        Ok(v) => v,
        Err(_) => default.to_string()
    }
}

pub fn save_string(key: &str, value: &str) -> Result<()> {
    match load_string(key) {
        Ok(s) if s == value => {}
        _ => {
            console::log_1(&format!("updating string key:{}, value:{}", key, value).into());
        }
    };
    let s = storage();
    match s.set_item(key, value) {
        Ok(()) => Ok(()),
        Err(e) => Err(anyhow!("{:?}", e))
    }
}