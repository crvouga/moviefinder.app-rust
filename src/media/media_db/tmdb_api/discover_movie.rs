use crate::{
    core::http,
    media::{genre::genre_id::GenreId, media::Media, media_id::MediaId, media_type::MediaType},
};

// https://developer.themoviedb.org/reference/discover-movie
use super::{
    config::{to_backdrop_image_set, to_poster_image_set, TmdbConfig},
    to_get_request, Config,
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

impl From<(&TmdbConfig, DiscoverMovieResult)> for Media {
    fn from((config, result): (&TmdbConfig, DiscoverMovieResult)) -> Self {
        Media {
            media_id: MediaId::new(result.id.unwrap_or(0.0).to_string()),
            media_backdrop: to_backdrop_image_set(
                config,
                result.backdrop_path.unwrap_or("".to_string()).as_str(),
            ),
            media_description: result.overview.unwrap_or("".to_string()),
            media_genre_ids: result
                .genre_ids
                .unwrap_or(vec![])
                .iter()
                .map(|id| GenreId::new(id.to_string()))
                .collect(),
            media_popularity: result.popularity.unwrap_or(0.0),
            media_poster: to_poster_image_set(
                config,
                result.poster_path.unwrap_or("".to_string()).as_str(),
            ),
            media_title: result.title.unwrap_or("".to_string()),
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
    let req = to_get_request(config, "/3/discover/movie");

    let sent = http::client::send(req).await;

    let response = match sent {
        Ok(response) => response,
        Err(err) => return Err(err.to_string()),
    };

    

    match serde_json::from_str(&response.body) {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(format!("Error parsing response: {} {}", e, response.body)),
    }
}
