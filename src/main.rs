use std::error::Error;

use serde::Deserialize;
use colour::{dark_green, yellow};
use ureq;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

impl Articles {
    fn render (&self) {
        for article in &self.articles {
            article.render();
        }
    }
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

impl Article {
    fn render (&self) {
        dark_green!("\n> {}\n--------------------------------------------------------------------------------------------------------\n", self.title);
        yellow!("\t{}\n\n", self.url);
    }
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response: String = ureq::get(url).call().unwrap().into_string().unwrap();

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/top-headlines?country=ro&apiKey=040ac743b01e4758bb8e8056130a3dd9";
    let articles = get_articles(url);
    let art: Articles;
    if let Ok(a) = articles {
        art = a;
    } else {
        panic!();
    }
    art.render();

    Ok(())
}
