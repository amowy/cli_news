use newsapi::{NewsApi, Endpoint, Country};
use std::error::Error;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsApi::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Hungary).fetch()?.write_to_terminal();

    Ok(())
}
