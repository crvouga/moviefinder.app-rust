use crate::http;

// https://developer.themoviedb.org/reference/discover-movie
use super::{to_base_headers, Config, HOST};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverMovieResult {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<i32>>,
    pub id: Option<i32>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverMovieResponse {
    pub page: Option<i32>,
    pub results: Option<Vec<DiscoverMovieResult>>,
    pub total_pages: Option<i32>,
    pub total_results: Option<i32>,
}

pub async fn send(config: &Config) -> Result<DiscoverMovieResponse, String> {
    let sent = http::client::send(http::Request {
        headers: to_base_headers(config),
        host: HOST.to_string(),
        method: "GET".to_string(),
        path: "/3/discover/movie".to_string(),
    })
    .await;

    match sent {
        Ok(response) => {
            let body = response.body;
            println!("{}", body);
            let parsed = serde_json::from_str(&body);
            match parsed {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(format!("Error parsing response: {}", e)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
