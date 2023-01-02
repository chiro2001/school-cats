use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostsPost {
    pub text: String,
    pub images: Vec<String>,
    pub places: Vec<u32>,
}