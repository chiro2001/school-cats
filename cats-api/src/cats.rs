#![allow(non_snake_case)]

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::user::User;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CatDisp {
    pub catId: u32,
    pub breed: BreedDB,
    pub name: String,
    pub foundTime: SystemTime,
    pub source: String,
    pub atSchool: bool,
    pub whereabouts: String,
    pub health: String,
}

impl Default for CatDisp {
    fn default() -> Self {
        Self {
            catId: 0,
            breed: BreedDB::default(),
            name: "".to_string(),
            foundTime: UNIX_EPOCH,
            source: "".to_string(),
            atSchool: false,
            whereabouts: "".to_string(),
            health: "".to_string(),
        }
    }
}

impl CatDisp {
    pub fn from_db(c: CatDB, breed: BreedDB) -> Self {
        Self {
            catId: c.catId,
            breed,
            name: c.name,
            foundTime: c.foundTime,
            source: c.source,
            atSchool: c.atSchool,
            whereabouts: c.whereabouts,
            health: c.health,
        }
    }
}

impl CatDB {
    pub fn copy(&self) -> Self {
        Self {
            catId: self.catId,
            breedId: self.breedId,
            name: self.name.to_string(),
            foundTime: self.foundTime.clone(),
            source: self.source.to_string(),
            atSchool: self.atSchool,
            whereabouts: self.whereabouts.to_string(),
            health: self.health.to_string(),
        }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BreedDB {
    pub breedId: u32,
    pub breedName: String,
    pub breedDesc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BreedPost {
    pub name: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedingDB {
    pub catId: u32,
    pub userId: u32,
    pub placeId: u32,
    pub feedTime: SystemTime,
    pub feedFood: String,
    pub feedAmount: String,
}

impl Default for FeedingDB {
    fn default() -> Self {
        Self {
            catId: 0,
            userId: 0,
            placeId: 0,
            feedTime: UNIX_EPOCH,
            feedFood: "".to_string(),
            feedAmount: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeedingInfo {
    pub cat: CatDisp,
    pub last: FeedingDB,
    pub user: User,
}