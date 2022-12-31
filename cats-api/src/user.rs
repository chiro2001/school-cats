use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub uid: u32,
    pub usernick: String,
    pub motto: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct LoginPost {
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct RegisterPost {
    pub user: User,
    pub password: String,
}