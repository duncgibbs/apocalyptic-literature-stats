use std::collections::HashMap;
use std::fs::{OpenOptions, create_dir_all};
use std::io::prelude::*;
use serde_json;

pub fn write_popular_by_year(year_data: &HashMap<i32, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving popular_by_year.json...");
    create_dir_all("./goodreads")?;

    let mut years_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./goodreads/popular_by_year.json")?;

    let years_string = serde_json::to_string(year_data)?;

    years_file.write_all(years_string.as_bytes())?;

    Ok(())
}

pub fn get_popular_by_year() -> Result<HashMap<i32, Vec<String>>, Box<dyn std::error::Error>> {
    create_dir_all("./goodreads")?;

    let mut popular_by_year_string = String::new();

    let mut popular_by_year_file = OpenOptions::new()
        .read(true)
        .open("./goodreads/popular_by_year.json")?;

    popular_by_year_file.read_to_string(&mut popular_by_year_string)?;
    let mut popular_by_year: HashMap<i32, Vec<String>> = HashMap::new();
    
    if popular_by_year_string.len() > 0 {
        popular_by_year = serde_json::from_str(&popular_by_year_string)?;
    }

    Ok(popular_by_year)
}
