use crate::{
    core::http,
    media::{genre::genre_id::GenreId, media::Media, media_id::MediaId, media_type::MediaType},
};

// https://developer.themoviedb.org/reference/discover-movie
use super::{
    config::{to_backdrop_image_set, to_poster_image_set, TmdbConfig},
    to_base_headers, Config, HOST,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverMovieResult {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Option<Vec<f64>>,
    pub id: Option<f64>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<f64>,
}

impl DiscoverMovieResult {
    pub fn to_media(self, config: &TmdbConfig) -> Media {
        Media {
            media_id: MediaId::new(self.id.unwrap_or(0.0).to_string()),
            media_backdrop: to_backdrop_image_set(
                config,
                self.backdrop_path.unwrap_or("".to_string()).as_str(),
            ),
            media_description: self.overview.unwrap_or("".to_string()),
            media_genre_ids: self
                .genre_ids
                .unwrap_or(vec![])
                .iter()
                .map(|id| GenreId::new(id.to_string()))
                .collect(),
            media_popularity: self.popularity.unwrap_or(0.0),
            media_poster: to_poster_image_set(
                config,
                self.poster_path.unwrap_or("".to_string()).as_str(),
            ),
            media_title: self.title.unwrap_or("".to_string()),
            media_type: MediaType::Movie,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverMovieResponse {
    pub page: Option<u32>,
    pub results: Option<Vec<DiscoverMovieResult>>,
    pub total_pages: Option<u32>,
    pub total_results: Option<u32>,
}

pub async fn send(config: &Config) -> Result<DiscoverMovieResponse, String> {
    let req = http::Request {
        headers: to_base_headers(config),
        host: HOST.to_string(),
        method: "GET".to_string(),
        path: "/3/discover/movie".to_string(),
    };

    let sent = http::client::send(req).await;

    match sent {
        Ok(response) => {
            let body = response.body;
            let parsed: Result<DiscoverMovieResponse, serde_json::Error> =
                serde_json::from_str(&body);
            match parsed {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(format!("Error parsing response: {} {}", e, body)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
