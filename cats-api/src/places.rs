use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PlacePost {
    pub details: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceDB {
    pub id: u32,
    pub details: String,
}

impl PlaceDB {
    pub fn copy(&self) -> Self {
        Self { id: self.id, details: self.details.to_string() }
    }
}

