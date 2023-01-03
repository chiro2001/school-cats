#![allow(non_snake_case)]

use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatDB {
    pub catId: u32,
    pub breedId: u32,
    pub name: String,
    pub foundTime: SystemTime,
    pub source: String,
    pub atSchool: bool,
    pub whereabouts: String,
    pub health: String,
}

impl Default for CatDB {
    fn default() -> Self {
        Self {
            catId: 1,
            breedId: 1,
            name: "喵".to_string(),
            foundTime: SystemTime::UNIX_EPOCH,
            source: "".to_string(),
            atSchool: false,
            whereabouts: "".to_string(),
            health: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatPlacesResponse {
    pub cat: CatDB,
    pub places: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BreedDB {
    pub breedId: u32,
    pub breedName: String,
    pub breedDesc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BreedPost {
    pub name: String,
    pub desc: String,
}