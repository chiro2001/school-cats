use serde::{Serialize, Deserialize};

pub mod user;
pub mod jwt;

pub const PORT: u16 = 3030;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct Hello {
    pub msg: String,
}

#[cfg(test)]
mod tests {
    use super::*;
}
