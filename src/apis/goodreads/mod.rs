pub mod shelf;
pub mod year;

use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub rating: f32,
    pub num_ratings: i64,
    pub year_published: i32,
    pub book_url: String,
    pub editions: Vec<i32>,
    pub shelf_num: i32,
}

#[derive(Serialize, Deserialize, Debug, Eq)]
pub struct Shelf {
    pub name: String,
    pub total: i32,
}

impl PartialEq for Shelf {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
