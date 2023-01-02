#![allow(non_snake_case)]

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::cats::CatDB;
use crate::user::User;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostsPost {
    pub text: String,
    pub images: Vec<String>,
    pub places: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostsContentDB {
    pub postId: u32,
    pub userId: u32,
    pub postTime: SystemTime,
    pub postText: String,
}

impl Default for PostsContentDB {
    fn default() -> Self {
        Self {
            postId: 0,
            userId: 0,
            postTime: UNIX_EPOCH,
            postText: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CommentDB {
    pub commentId: u32,
    pub userId: u32,
    pub commentText: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CommentDisp {
    pub user: User,
    pub text: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostDisp {
    pub postId: u32,
    pub user: User,
    pub images: Vec<String>,
    pub comments: Vec<CommentDisp>,
    pub places: Vec<String>,
    pub cats: Vec<CatDB>,
}