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
    pub cats: Vec<u32>,
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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct CommentDisp {
    pub user: User,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostDisp {
    pub postId: u32,
    pub text: String,
    pub time: SystemTime,
    pub user: User,
    pub images: Vec<String>,
    pub comments: Vec<CommentDisp>,
    pub places: Vec<String>,
    pub cats: Vec<CatDB>,
}

impl PostDisp {
    pub fn copy(&self) -> Self {
        Self{
            postId: self.postId,
            text: self.text.to_string(),
            time: self.time,
            user: self.user.clone(),
            images: self.images.clone(),
            comments: self.comments.clone(),
            places: self.places.clone(),
            cats: self.cats.clone(),
        }
    }
}

impl Default for PostDisp {
    fn default() -> Self {
        Self {
            postId: 0,
            text: "".to_string(),
            time: SystemTime::UNIX_EPOCH,
            user: Default::default(),
            images: vec![],
            comments: vec![],
            places: vec![],
            cats: vec![],
        }
    }
}

impl PartialEq for PostDisp {
    fn eq(&self, other: &Self) -> bool {
        self.postId == other.postId
    }
}