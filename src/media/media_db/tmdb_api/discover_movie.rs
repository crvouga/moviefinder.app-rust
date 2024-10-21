// use crate::http;

// https://developer.themoviedb.org/reference/discover-movie
use super::Config;
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

pub fn send(config: &Config) -> Result<DiscoverMovieResponse, String> {
    // http::client::send(request)
    println!("discover_movie::send: {:?}", config);
    Err("Not implemented".to_string())
}
