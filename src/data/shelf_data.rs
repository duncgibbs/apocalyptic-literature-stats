use crate::apis::goodreads::{Book, Shelf};
use std::fs::{OpenOptions, create_dir_all};
use std::io::prelude::*;
use serde_json;

pub fn write_shelf_results(shelf_name: &str, books: Vec<Book>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving books...");
    create_dir_all(format!("./goodreads/shelves/{}", shelf_name))?;

    let mut books_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("./goodreads/shelves/{}/books.json", shelf_name))?;

    let books_string = serde_json::to_string(&books)?;

    books_file.write_all(books_string.as_bytes())?;

    Ok(())
}

pub fn write_shelf(shelf: Shelf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving shelf...");
    create_dir_all("./goodreads/shelves")?;

    let mut existing_shelves_string = String::new();

    let mut existing_shelves_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("./goodreads/shelves/genres.json")?;

    existing_shelves_file.read_to_string(&mut existing_shelves_string)?;
    let mut shelves: Vec<Shelf> = Vec::new();
    
    if existing_shelves_string.len() > 0 {
        shelves = serde_json::from_str(&existing_shelves_string)?;
    }

    if shelves.contains(&shelf) {
        shelves.retain(|old_shelf| old_shelf != &shelf);
    }

    shelves.push(shelf);

    let mut new_shelves_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./goodreads/shelves/genres.json")?;

    let new_shelves_string = serde_json::to_string(&shelves)?;
    new_shelves_file.write_all(new_shelves_string.as_bytes())?;
    Ok(())
}

pub fn get_shelf_books(shelf_name: &str) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
    let mut books: Vec<Book> = Vec::new();
    
    let mut books_string = String::new();
    
    let mut books_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(format!("./goodreads/shelves/{}/books.json", shelf_name))?;

    books_file.read_to_string(&mut books_string)?;

    if books_string.len() > 0 {
        books = serde_json::from_str(&books_string)?;
    }

    Ok(books)
}

pub fn get_shelf(shelf_name: &str) -> Result<Shelf, Box<dyn std::error::Error>> {
    let mut existing_shelves_string = String::new();

    let mut existing_shelves_file = OpenOptions::new()
        .read(true)
        .open("./goodreads/shelves/genres.json")?;

    existing_shelves_file.read_to_string(&mut existing_shelves_string)?;
    let mut shelves: Vec<Shelf> = Vec::new();
    
    if existing_shelves_string.len() > 0 {
        shelves = serde_json::from_str(&existing_shelves_string)?;
    }

    let shelf = shelves.into_iter().find(|shelf| shelf.name == shelf_name).unwrap();

    Ok(shelf)
}
