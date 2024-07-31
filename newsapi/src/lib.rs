mod theme;

use serde::Deserialize;
use termimad::MadSkin;
use ureq;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError{
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response into string")]
    ResponseToStringFailed(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    articles: Vec<Article>,                   
}

impl Articles {
    pub fn write (&self) {
        let theme = theme::default();
        theme.print_text("Major headlines\n");
        for article in &self.articles {
            article.write(&theme);
        }
    }
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

impl Article {
    fn write (&self, theme: &MadSkin) {
        theme.print_text(&format!("\n> {}\n--------------------------------------------------------------------------------------------------------\n", self.title));
        theme.print_text(&format!("\t{}\n\n", self.url));
    }
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response: String = ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))?
                            .into_string().map_err(|e| NewsApiError::ResponseToStringFailed(e))?;

    

    let articles: Articles = serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParseFailed(e))?;

    Ok(articles)
}

