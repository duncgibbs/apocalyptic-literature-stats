use crate::data::shelf_data;
use crate::data::year_data;
use crate::apis::goodreads::Book;

use std::collections::HashMap;
use std::fs::File;

pub fn print_books(books: Vec<Book>) {
    println!("EVCXR_BEGIN_CONTENT text/html\n");
    println!("<table style=\"width:100%\"><tbody>");
    println!("<tr><td><b>Title</b></td><td><b>Author</b></td><td><b>Year Published</b></td></tr>");
    
    for book in books {
        println!(
            "{}",
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                book.title,
                book.author,
                book.year_published
            )
        );
    }
    
    println!("</tbody></table>");
    println!("\nEVCXR_END_CONTENT");
}

pub fn print_books_with_editions(books: Vec<Book>) {
    println!("EVCXR_BEGIN_CONTENT text/html\n");
    println!("<table style=\"width:100%\"><tbody>");
    println!("<tr><td><b>Title</b></td><td><b>Author</b></td><td><b>Year Published</b></td><td><b>Number of Editions</b></td></tr>");
    
    for book in books {
        println!(
            "{}",
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                book.title,
                book.author,
                book.year_published,
                book.editions.len()
            )
        );
    }
    
    println!("</tbody></table>");
    println!("\nEVCXR_END_CONTENT");
}

pub fn print_books_with_ratings(books: Vec<Book>) {
    println!("EVCXR_BEGIN_CONTENT text/html\n");
    println!("<table style=\"width:100%\"><tbody>");
    println!("<tr><td><b>Title</b></td><td><b>Author</b></td><td><b>Year Published</b></td><td><b>Rating</b></td></tr>");
    
    for book in books {
        println!(
            "{}",
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                book.title,
                book.author,
                book.year_published,
                book.rating
            )
        );
    }
    
    println!("</tbody></table>");
    println!("\nEVCXR_END_CONTENT");
}

pub fn print_books_with_ratings_and_num_ratings(books: Vec<Book>) {
    println!("EVCXR_BEGIN_CONTENT text/html\n");
    println!("<table style=\"width:100%\"><tbody>");
    println!("<tr><td><b>Title</b></td><td><b>Author</b></td><td><b>Year Published</b></td><td><b>Rating</b></td><td><b>Number of Ratings</b></td></tr>");
    
    for book in books {
        println!(
            "{}",
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                book.title,
                book.author,
                book.year_published,
                book.rating,
                book.num_ratings
            )
        );
    }
    
    println!("</tbody></table>");
    println!("\nEVCXR_END_CONTENT");
}

pub fn print_books_with_weighted_ratings(books: Vec<Book>) {
    println!("EVCXR_BEGIN_CONTENT text/html\n");
    println!("<table style=\"width:100%\"><tbody>");
    println!("<tr><td><b>Title</b></td><td><b>Author</b></td><td><b>Year Published</b></td><td><b>Weighted Rating</b></td></tr>");
    
    for book in books {
        println!(
            "{}",
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                book.title,
                book.author,
                book.year_published,
                get_weighted_rating_for_book(&book)
            )
        );
    }
    
    println!("</tbody></table>");
    println!("\nEVCXR_END_CONTENT");
}

pub fn get_books(shelf_name: &str) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
    return shelf_data::get_shelf_books(shelf_name);
}

pub fn get_books_per_year_map(shelf_name: &str) -> Result<HashMap<i32, i32>, Box<dyn std::error::Error>> {
    let books = shelf_data::get_shelf_books(shelf_name)?;

    let mut books_per_year: HashMap<i32, i32> = HashMap::new();

    for book in books {
        let count = books_per_year.entry(book.year_published).or_insert(0);
        *count += 1;
    }

    Ok(books_per_year)
}

pub fn get_books_per_year_vec(shelf_name: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let books_per_year = get_books_per_year_map(shelf_name)?;

    let mut books_per_year_vec: Vec<(i32, i32)> = books_per_year.into_iter().collect();
    books_per_year_vec.retain(|result| result.0 > 0);
    books_per_year_vec.sort_by(|result_one, result_two| result_one.0.cmp(&result_two.0));

    Ok(books_per_year_vec)
}

pub fn get_books_per_year_percent(shelf_name: &str, total: i32) -> Result<Vec<(i32, f64)>, Box<dyn std::error::Error>> {
    let books_per_year = get_books_per_year_map(shelf_name)?;
    let books_per_year_vec = get_books_per_year_vec(shelf_name)?;

    let first_year = books_per_year_vec[0].0;
    let last_year = books_per_year_vec.last().unwrap().0;

    let data: Vec<(i32, f64)> = (first_year..last_year).map(|year| {
        let num_that_year = books_per_year.get(&year).unwrap_or(&0);
        let percent_of_total: f64 = *num_that_year as f64 / total as f64;
        (year, percent_of_total)
    }).collect();
    
    Ok(data)
}

pub fn get_books_per_year_percent_partial(shelf_name: &str) -> Result<Vec<(i32, f64)>, Box<dyn std::error::Error>> {
    let shelf_total = shelf_data::get_shelf_books(shelf_name)?.len() as i32;
    Ok(get_books_per_year_percent(shelf_name, shelf_total)?)
}

pub fn get_books_per_year_percent_total(shelf_name: &str) -> Result<Vec<(i32, f64)>, Box<dyn std::error::Error>> {
    let shelf_total = crate::data::shelf_data::get_shelf("apocalyptic")?.total;
    Ok(get_books_per_year_percent(shelf_name, shelf_total)?)
}

fn get_number_of_top_x(shelf_name: &str, top: usize, bottom: usize) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let books = shelf_data::get_shelf_books(shelf_name)?;
    let popular_by_year = year_data::get_popular_by_year()?;

    let mut results: Vec<(i32, i32)> = Vec::new();

    let book_titles: Vec<String> = books.into_iter().map(|book| book.title).collect();

    for year in popular_by_year.keys() {
        match popular_by_year.get(&year) {
            Some(popular) => {
                let number_of_popular = popular[top..bottom]
                .into_iter()
                .filter(|title| book_titles.contains(&title))
                .collect::<Vec<&String>>()
                .len() as i32;
                results.push((*year, number_of_popular));
            },
            _ => ()
        }
    }
    
    Ok(results)
}

pub fn get_number_of_top_200(shelf_name: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    Ok(get_number_of_top_x(shelf_name, 0, 199)?)
}

pub fn get_number_of_top_10(shelf_name: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    Ok(get_number_of_top_x(shelf_name, 0, 10)?)
}

pub fn get_number_of_bottom_190(shelf_name: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    Ok(get_number_of_top_x(shelf_name, 10, 199)?)
}

pub fn get_editions_per_year(editions: Vec<i32>) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let mut editions_per_year: HashMap<i32, i32> = HashMap::new();

    let mut start_year = *editions.iter().min().unwrap();

    if start_year < 1945 {
        start_year = 1945;
    }

    for year in start_year..2021 {
        editions_per_year.insert(year, editions.iter().filter(|&x| *x == year).count() as i32);
    }

    let mut sorted_editions: Vec<(i32, i32)> = editions_per_year.into_iter().collect();
    sorted_editions.sort_by(|edition_one, edition_two| edition_one.0.cmp(&edition_two.0));

    Ok(sorted_editions)
}

pub fn get_editions_per_year_3d(editions: Vec<i32>, year_published: i32) -> Result<Vec<(i32, i32, i32)>, Box<dyn std::error::Error>> {
    let mut filtered_editions = editions;
    filtered_editions.retain(|edition| edition >= &year_published);
    
    let sorted_editions = get_editions_per_year(filtered_editions)?;

    let sorted_editions_by_year_published: Vec<(i32, i32, i32)> = sorted_editions.iter().map(|(x, y)| (*x, *y, year_published)).collect();

    Ok(sorted_editions_by_year_published)
}

pub fn get_editions_per_year_surface(shelf_name: &str) -> Result<HashMap<(i32, i32), i32>, Box<dyn std::error::Error>> {
    let mut books = shelf_data::get_shelf_books(shelf_name)?;
    books.retain(|book| book.year_published > 1944);
    books.sort_by(|book_one, book_two| book_two.year_published.cmp(&book_one.year_published));

    let mut results: HashMap<(i32, i32), i32> = HashMap::new();

    for book in books {
        let editions_by_year = get_editions_per_year_3d(book.editions, book.year_published)?;
        for (x, y, z) in editions_by_year {
            let count = results.entry((x, z)).or_insert(0);
            *count += y;
        }
    }

    Ok(results)
}

pub fn get_goodreads_google_trends() -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let file = File::open("./goodreads-google-trends.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut trends_map: HashMap<i32, i32> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let year_month = &record[0];
        let year: i32 = year_month.split("-").collect::<Vec<&str>>()[0].parse()?;
        let trend: i32 = record[1].parse()?;
        
        let count = trends_map.entry(year).or_insert(0);
        *count += trend;
    }

    let trends: Vec<(i32, i32)> = trends_map.into_iter().collect();

    Ok(trends)
}

pub fn get_shelf_rating_per_year(books: &Vec<Book>) -> Result<HashMap<i32,f32>, Box<dyn std::error::Error>> {
    let mut ratings_by_year_published: HashMap<i32,Vec<f32>> = HashMap::new();

    for book in books {
        let rating = ratings_by_year_published.entry(book.year_published).or_insert(Vec::new());
        rating.push(book.rating);
    }

    let mut results: HashMap<i32,f32> = HashMap::new();

    for year in ratings_by_year_published.keys() {
        let ratings = ratings_by_year_published.get(year).unwrap();
        let average_rating = ratings.into_iter().sum::<f32>() / ratings.len() as f32;
        results.insert(*year, average_rating);
    }

    Ok(results)
}

pub fn get_weighted_rating_for_book(book: &Book) -> f32 {
    // 6060 is our median num_ratings on a book
    // (63362 is our average num_ratings)
    // 3.923561 is our average rating
    let weighted_rating = (book.num_ratings as f32 / (book.num_ratings + 6060) as f32) * book.rating + (6060 as f32 / (book.num_ratings + 6060) as f32) * 3.923561;
    return weighted_rating;
}

pub fn get_shelf_weighted_rating_per_year(books: &Vec<Book>) -> Result<HashMap<i32,f32>, Box<dyn std::error::Error>> {
    let mut ratings_by_year_published: HashMap<i32,Vec<f32>> = HashMap::new();

    for book in books {
        let rating = ratings_by_year_published.entry(book.year_published).or_insert(Vec::new());
        rating.push(get_weighted_rating_for_book(book));
    }

    let mut results: HashMap<i32,f32> = HashMap::new();

    for year in ratings_by_year_published.keys() {
        let ratings = ratings_by_year_published.get(year).unwrap();
        let average_rating = ratings.into_iter().sum::<f32>() / ratings.len() as f32;
        results.insert(*year, average_rating);
    }

    Ok(results)
}

pub fn get_shelf_weighted_rating_for_years(books: &Vec<Book>, min_year: i32, max_year: i32) -> Result<f32, Box<dyn std::error::Error>> {
    let weighted_ratings = get_shelf_weighted_rating_per_year(books)?;

    let mut results: Vec<f32> = Vec::new();

    let year_range = (min_year..max_year + 1).filter(|year| weighted_ratings.contains_key(year));

    for year in year_range {
        let weighted_rating = *weighted_ratings.get(&year).unwrap();
        results.push(weighted_rating);
    }

    let num_years = results.len() as f32;

    Ok(results.into_iter().sum::<f32>() / num_years)
}

pub fn get_book_shelf_percentage(book: &Book) -> f32 {
    let shelves = book.shelf_num as f32;
    let num_ratings = book.num_ratings as f32;

    return (shelves / num_ratings) * 100.0;
}

pub fn get_shelf_shelved_percentage_per_year(books: &Vec<Book>) -> Result<HashMap<i32,f32>, Box<dyn std::error::Error>> {
    let mut shelves_by_year_published: HashMap<i32,Vec<f32>> = HashMap::new();

    for book in books {
        let rating = shelves_by_year_published.entry(book.year_published).or_insert(Vec::new());
        rating.push(get_book_shelf_percentage(&book));
    }

    let mut results: HashMap<i32,f32> = HashMap::new();

    for year in shelves_by_year_published.keys() {
        let shelves = shelves_by_year_published.get(year).unwrap();
        let average_shelf_percent = shelves.into_iter().sum::<f32>() / shelves.len() as f32;
        results.insert(*year, average_shelf_percent);
    }

    Ok(results)
}

pub fn get_shelf_percentage_for_years(books: &Vec<Book>, min_year: i32, max_year: i32) -> Result<f32, Box<dyn std::error::Error>> {
    let shelf_percentages = get_shelf_shelved_percentage_per_year(books)?;

    let mut results: Vec<f32> = Vec::new();

    let year_range = (min_year..max_year + 1).filter(|year| shelf_percentages.contains_key(year));

    for year in year_range {
        let shelf_percentage = *shelf_percentages.get(&year).unwrap();
        results.push(shelf_percentage);
    }

    let num_years = results.len() as f32;

    Ok(results.into_iter().sum::<f32>() / num_years)
}

pub fn get_shelf_num_ratings_per_year(books: &Vec<Book>) -> Result<HashMap<i32,i32>, Box<dyn std::error::Error>> {
    let mut shelves_by_year_published: HashMap<i32,Vec<i64>> = HashMap::new();

    for book in books {
        let rating = shelves_by_year_published.entry(book.year_published).or_insert(Vec::new());
        rating.push(book.num_ratings);
    }

    let mut results: HashMap<i32,i32> = HashMap::new();

    for year in shelves_by_year_published.keys() {
        let shelves = shelves_by_year_published.get(year).unwrap();
        let average_shelf_percent = shelves.into_iter().sum::<i64>() / shelves.len() as i64;
        results.insert(*year, average_shelf_percent as i32);
    }

    Ok(results)
}

pub fn get_num_ratings_for_years(books: &Vec<Book>, min_year: i32, max_year: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let num_ratings = get_shelf_num_ratings_per_year(books)?;

    let mut results: Vec<i32> = Vec::new();

    let year_range = (min_year..max_year + 1).filter(|year| num_ratings.contains_key(year));

    for year in year_range {
        let num = *num_ratings.get(&year).unwrap();
        results.push(num);
    }

    let num_years = results.len() as i32;

    Ok(results.into_iter().sum::<i32>() / num_years)
}

pub fn get_shelf_num_per_year(books: &Vec<Book>) -> Result<HashMap<i32,i32>, Box<dyn std::error::Error>> {
    let mut shelves_by_year_published: HashMap<i32,Vec<i32>> = HashMap::new();

    for book in books {
        let rating = shelves_by_year_published.entry(book.year_published).or_insert(Vec::new());
        rating.push(book.shelf_num);
    }

    let mut results: HashMap<i32,i32> = HashMap::new();

    for year in shelves_by_year_published.keys() {
        let shelves = shelves_by_year_published.get(year).unwrap();
        let average_shelf_percent = shelves.into_iter().sum::<i32>() / shelves.len() as i32;
        results.insert(*year, average_shelf_percent as i32);
    }

    Ok(results)
}

pub fn get_shelf_num_for_years(books: &Vec<Book>, min_year: i32, max_year: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let shelf_nums = get_shelf_num_per_year(books)?;

    let mut results: Vec<i32> = Vec::new();

    let year_range = (min_year..max_year + 1).filter(|year| shelf_nums.contains_key(year));

    for year in year_range {
        let num = *shelf_nums.get(&year).unwrap();
        results.push(num);
    }

    let num_years = results.len() as i32;

    Ok(results.into_iter().sum::<i32>() / num_years)
}
