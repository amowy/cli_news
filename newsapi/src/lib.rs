mod theme;

use serde::Deserialize;
use serde_json;
use termimad::MadSkin;
use ureq;
use url::Url;
#[cfg(feature = "async")]
use reqwest::Method;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError{
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response into string")]
    ResponseToStringFailed(#[from] std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(#[from] serde_json::Error),
    #[error("Url Parsing Failed")]
    UrlParsingFailed(#[from] url::ParseError),
    #[error("Request failed {0}")]
    BadRequest(&'static str),
    #[cfg(feature = "async")]
    #[error("Url Parsing Failed")]
    AsyncRequestFailed(#[from] reqwest::Error)
}

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    status: String,
    articles: Vec<Article>,
    code: Option<String>,
}

impl NewsApiResponse {
    pub fn write_to_terminal (&self) {
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

/* 
pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response: String = ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))?
                            .into_string().map_err(|e| NewsApiError::ResponseToStringFailed(e))?;

    

    let articles: Articles = serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParseFailed(e))?;

    Ok(articles)
}
*/
 
#[derive(Debug)]
pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => String::from("top-headlines"),
        }
    }
}

#[derive(Debug)]
pub enum Country {
    Us,
    Romania,
    Hungary,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Us => String::from("us"),
            Self::Romania => String::from("ro"),
            Self::Hungary => String::from("hu")
        }
    }
}

#[derive(Debug)]
pub struct NewsApi {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsApi {
    pub fn new (api_key: &str) -> NewsApi {
        NewsApi {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::Us,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsApi {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsApi {
        self.country = country;
        self
    }

    fn prepare_url(&self)  -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());
        let country = format!("country={}",self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsApiResponse = req.call()?.into_json()?;
        
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_error(response.code))
        }
    }

    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) ->  Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::new();

        let request = client
        .request(Method::Get, url)
        .header("Authorization", &self.api_key)
        .build()
        .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let response: NewsApiResponse = client.execute(request).await?.json().await.map_err(|e| NewsApiError::AsyncRequestFailed(e))?;
    
        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_error(response.code))
        }
    }

}

fn map_response_error(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled."),
            _ => NewsApiError::BadRequest("Unknown Error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown Error")
    }
}