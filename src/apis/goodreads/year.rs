use crate::data::year_data;

use fantoccini::{Client, Locator};
use std::collections::HashMap;

pub async fn scrape_years() -> Result<(), Box<dyn std::error::Error>> {
    let mut results: HashMap<i32, Vec<String>> = HashMap::new();
    let mut client = Client::new("http://localhost:9515").await.expect("failed to connect to WebDriver");

    for year in 1900..2021 {
        println!("Scraping most popular books from {}...", year);
        results.insert(year, Vec::new());
        let year_url = format!("https://www.goodreads.com/book/popular_by_date/{}", year);
        client.goto(&year_url).await?;

        for mut span in client.find_all(Locator::Css("a.bookTitle > span")).await? {
            let mut book_list = results.remove(&year).unwrap();
            book_list.push(span.html(true).await?);
            results.insert(year, book_list);
        }
    }

    year_data::write_popular_by_year(&results)?;

    client.close().await?;

    Ok(())
}