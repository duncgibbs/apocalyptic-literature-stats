use std::collections::HashMap;
use std::fs::{OpenOptions, create_dir_all};
use std::io::prelude::*;
use serde_json;

pub fn write_popular_by_year(year_data: &HashMap<u16, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving popular_by_year.json...");
    create_dir_all(format!("./goodreads"))?;

    let mut years_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("./goodreads/popular_by_year.json"))?;

    let years_string = serde_json::to_string(year_data)?;

    years_file.write_all(years_string.as_bytes())?;

    Ok(())
}