mod apis;
mod data;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => println!("Specify a scrape target:\n\tshelf [shelf_name]\n\tpopular_by_year"),
        Some(target) => {
            match target.as_str() {
                "shelf" => scrape_shelf(Some(target)).await,
                "popular_by_year" => scrape_popular_by_year().await,
                &_ => println!("No command found for {}", target)
            }
        }
    }
}

async fn scrape_shelf(shelf_name: Option<&str>) {
    match shelf_name {
        None => println!("Specify a genre or shelf to scrape"),
        Some(shelf_name_string) => {
            match apis::goodreads::shelf::scrape_shelf(shelf_name_string).await {
                Ok(_) => println!("Done."),
                Err(err) => println!("Error: {:?}", err)
            }
        }
    }
}

async fn scrape_popular_by_year() {
    match apis::goodreads::year::scrape_years().await {
        Ok(_) => println!("Done."),
        Err(err) => println!("Error: {:?}", err)
    }
}