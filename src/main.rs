use newsapi::{Articles, get_articles};
use std::error::Error;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;

    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}",url, api_key);

    let articles: Articles = get_articles(&url)?;
    
    articles.write();

    Ok(())
}
