pub mod shelf;
pub mod year;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub rating: f32,
    pub num_ratings: u64,
    pub year_published: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Shelf {
    pub name: String,
    pub total: u32,
}

impl PartialEq for Shelf {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
