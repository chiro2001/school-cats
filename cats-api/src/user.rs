#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub uid: u32,
    pub head: u32,
    pub usernick: String,
    pub motto: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct UserDB {
    pub userId: u32,
    pub username: String,
    pub passwd: String,
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
            head: u.imageId,
        }
    }
}

impl UserDB {
    pub fn from_user(u: User, uid: u32, passwd: &str) -> Self {
        Self {
            userId: uid,
            username: u.username,
            imageId: u.head,
            usernick: u.usernick,
            motto: u.motto,
            passwd: passwd.to_string(),
        }
    }
    pub fn remove_passwd(self) -> Self {
        Self {
            passwd: "".to_string(),
            ..self
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct LoginPost {
    pub username: String,
    pub passwd: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct RegisterPost {
    pub user: User,
    pub passwd: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct ImageDB {
    pub url: String,
    pub imageId: u32,
}
