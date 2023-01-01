use serde::{Serialize, Deserialize};

pub mod user;
pub mod jwt;

pub const PORT: u16 = 3030;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct Hello {
    pub msg: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Empty;

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    pub code: usize,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(code: usize, msg: &str, data: T) -> Self {
        Self { code, msg: msg.to_string(), data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(200, "ok", data)
    }

    pub fn error(msg: &str, data: T) -> Self {
        Self::new(400, msg, data)
    }
}

impl Response<Empty> {
    pub fn err() -> Self {
        Self::new(400, "error", Empty::default())
    }
}