use crate::data::shelf_data;
use crate::apis::goodreads::{Book, Shelf};

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use regex::Regex;
use fantoccini::{Client, Locator};

pub async fn scrape_shelf(shelf_name: &str,) -> Result<(), Box<dyn std::error::Error>> {    
    let mut client = Client::new("http://localhost:9515").await.expect("failed to connect to WebDriver");

    client = login(client).await?;

    let shelf_results = get_shelf_total(client, shelf_name).await?;
    client = shelf_results.0;

    let page_results = get_number_of_pages(client, shelf_name).await?;
    client = page_results.0;
    let num_pages = page_results.1;

    let mut books: Vec<Book> = Vec::new();

    for page in 1..num_pages+1 {
        println!("Scraping page {} of {}...", page, num_pages);
        let book_results = scrape_shelf_page(client, shelf_name, page).await?;
        client = book_results.0;
        books.extend(book_results.1);
    }

    for book in books.iter_mut() {
        client = get_editions(client, book).await?;
    }

    client.close().await?;

    match shelf_results.1 {
        None => println!("Unable to find total for shelf: {}", shelf_name),
        Some(shelf) => {
            shelf_data::write_shelf(shelf)?;
        }
    }

    shelf_data::write_shelf_results(shelf_name, books)?;

    Ok(())
}

async fn get_shelf_total(mut client: Client, shelf_name: &str) -> Result<(Client, Option<Shelf>), Box<dyn std::error::Error>> {
    println!("Getting totals...");
    let shelf_url = format!("https://www.goodreads.com/shelf/show/{}", shelf_name);
    client.goto(&shelf_url).await?;

    let mut total_div = client.wait_for_find(Locator::Css("div.leftContainer > div.mediumText > span.smallText")).await?;
    let total_regex = Regex::new(r"(?s:.)* of (?P<total>[0-9,]+)")?;
    match total_regex.captures(&total_div.html(true).await?) {
        None => {
            println!("Error getting shelf total: {}", shelf_name);
            Ok((client, None))
        },
        Some(caps) => {
            let total = caps["total"].split(",").collect::<Vec<&str>>().join("");
            let shelf = Shelf {
                name: String::from(shelf_name),
                total: total.parse()?,
            };
        
            Ok((client, Some(shelf)))
        }
    }
}

async fn get_number_of_pages(mut client: Client, shelf_name: &str) -> Result<(Client, u8), Box<dyn std::error::Error>> {
    println!("Getting number of pages...");
    let shelf_url = format!("https://www.goodreads.com/shelf/show/{}", shelf_name);
    client.goto(&shelf_url).await?;

    let mut pagination_div = client.wait_for_find(Locator::Css("div[max_num_pages]")).await?;

    match pagination_div.attr("max_num_pages").await? {
        None => {
            println!("Unable to get number of pages for shelf {}", shelf_name);
            Ok((client, 0))
        },
        Some(max_num_pages) => Ok((client, max_num_pages.parse()?))
    }    
}

async fn scrape_shelf_page(mut client: Client, shelf_name: &str, page: u8) -> Result<(Client, Vec<Book>), Box<dyn std::error::Error>> {
    let shelf_url = format!("https://www.goodreads.com/shelf/show/{}?page={}", shelf_name, page);
    
    client.goto(&shelf_url).await?;

    client.wait_for_find(Locator::Css("div.leftContainer")).await?;

    let book_elements = client.find_all(Locator::Css("div.leftContainer > div.elementList")).await?;

    let details_regex = Regex::new(r"(?x)
        (?s:.)*avg\ rating\ (?P<rating>[0-9.]+)
        (?s:\D)*(?P<num_ratings>[0-9,]+)\ ratings
        (?s:.)*published\ (?P<year_published>\d*)
    ")?;

    let shelved_regex = Regex::new(r"(?x)
        (?s:.)*\(shelved\ (?P<shelf_num>\d+)
    ")?;

    let mut book_results: Vec<Book> = Vec::new();

    for (idx, mut book) in book_elements.into_iter().enumerate() {
        let title = book.find(Locator::Css("a.leftAlignedImage")).await?
            .attr("title").await?.ok_or(
                format!(
                    "Error getting title for book '{}' on page '{}' on shelf '{}'",
                    idx,
                    page,
                    shelf_name,
                )
            )?;
        let book_url = book.find(Locator::Css("a.leftAlignedImage")).await?
            .attr("href").await?.ok_or(
                format!(
                    "Error getting url for book '{}' on page '{}' on shelf '{}'",
                    idx,
                    page,
                    shelf_name,
                )
            )?;
        let author = book.find(Locator::Css("a.authorName > span")).await?.html(true).await?;
        let shelved_link = book.find(Locator::Css("a.smallText")).await?.html(true).await?;
        let shelved_cap = shelved_regex.captures(&shelved_link).unwrap();
        let details_element = book.find(Locator::Css("div.left > span.greyText.smallText")).await?.html(true).await?;
        match details_regex.captures(&details_element) {
            None => {
                println!("error getting details: {}", details_element);
            },
            Some(details) => {
                let num_ratings = details["num_ratings"].split(",").collect::<Vec<&str>>().join("");
        
                book_results.push(Book {
                    title: title,
                    author: author,
                    rating: details["rating"].parse()?,
                    num_ratings: num_ratings.parse()?,
                    year_published: details["year_published"].parse().unwrap_or(0),
                    book_url: book_url,
                    editions: Vec::new(),
                    shelf_num: shelved_cap["shelf_num"].parse().unwrap_or(0),
                });
            }
        };
    }

    Ok((client, book_results))
}

async fn login(mut client: Client) -> Result<Client, Box<dyn std::error::Error>> {
    println!("Logging on...");
    dotenv().ok();

    let goodreads_url = format!("https://www.goodreads.com/");
    client.goto(&goodreads_url).await?;

    let username = env::var("GOODREADS_USERNAME").unwrap();
    let password = env::var("GOODREADS_PASSWORD").unwrap();

    client.wait_for_find(Locator::Css("input#userSignInFormEmail")).await?
    .send_keys(&username).await?;
    client.wait_for_find(Locator::Css("input#user_password")).await?
    .send_keys(&password).await?;

    client.wait_for_find(Locator::Css("input[type=\"submit\"]")).await?.click().await?;

    client.wait_for_find(Locator::Css("ul.personalNav")).await?;

    Ok(client)
}

async fn get_editions(mut client: Client, book: &mut Book) -> Result<Client, Box<dyn std::error::Error>> {
    println!("Getting editions for {}...", book.title);

    client.goto(&book.book_url).await?;

    let editions_link = client.wait_for_find(Locator::Css("div.otherEditionsActions > a")).await?;
    editions_link.click().await?;

    let mut next_page = true;

    while next_page {
        client.wait_for_find(Locator::Css("div.workEditions")).await?;

        let mut edition_data_rows = client.find_all(Locator::Css("div.editionData > div.dataRow")).await?;

        let year_regex = Regex::new(r"(?x)
            (?s:.)*Published\ (?s:.)*(?P<year_published>\d{4})
        ")?;

        for idx in 0..edition_data_rows.len() {
            let year_data = &edition_data_rows[idx].html(true).await?;
            match year_regex.captures(&year_data) {
                Some(year) => {
                    book.editions.push(year["year_published"].parse()?);
                },
                _ => ()
            };
        }

        match client.find(Locator::Css("a.next_page")).await {
            Ok(next_page_button) => {
                next_page_button.click().await?;
            },
            Err(_) => {
                next_page = false;
            }
        };
    }

    Ok(client)
}
