#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub uid: u32,
    pub usernick: String,
    pub motto: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct UserDB {
    pub userId: u32,
    pub username: String,
    pub imageId: u32,
    pub usernick: String,
    pub motto: String,
}

impl From<UserDB> for User {
    fn from(u: UserDB) -> Self {
        Self {
            username: u.username,
            uid: u.userId,
            usernick: u.usernick,
            motto: u.motto,
        }
    }
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

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct ImageDB {
    pub url: String,
    pub imageId: u32,
}
