#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use crate::user::User;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostsPost {
    pub text: String,
    pub images: Vec<String>,
    pub places: Vec<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostsDB {
    pub postId: u32,
    pub userId: u32,
    pub catId: u32,
    pub imageId: u32,
    pub placeId: u32,
    pub commentId: u32,
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
}