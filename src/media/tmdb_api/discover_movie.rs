use crate::{
    core::{
        http::{self, query_params::QueryParams},
        struct_ext::struct_to_map,
    },
    media::{core::Media, genre::genre_id::GenreId, media_id::MediaId, media_type::MediaType},
};

use super::{config::TmdbConfig, TmdbApi};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            media_backdrop: config
                .to_backdrop_image_set(result.backdrop_path.unwrap_or("".to_string()).as_str()),
            media_description: result.overview.unwrap_or("".to_string()),
            media_genre_ids: result
                .genre_ids
                .unwrap_or(vec![])
                .iter()
                .map(|id| GenreId::new(id.to_string()))
                .collect(),
            media_popularity: result.popularity.unwrap_or(0.0),
            media_poster: config
                .to_poster_image_set(result.poster_path.unwrap_or("".to_string()).as_str()),
            media_title: result.title.unwrap_or("".to_string()),
            media_type: MediaType::Movie,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoverMovieResponse {
    pub page: Option<usize>,
    pub results: Option<Vec<DiscoverMovieResult>>,
    pub total_pages: Option<usize>,
    pub total_results: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DiscoverMovieParams {
    pub language: Option<String>,
    pub region: Option<String>,
    pub sort_by: Option<String>,
    pub certification_country: Option<String>,
    pub certification: Option<String>,
    pub certification_lte: Option<String>,
    pub certification_gte: Option<String>,
    pub include_adult: Option<bool>,
    pub include_video: Option<bool>,
    pub page: Option<u32>,
    pub primary_release_year: Option<u32>,
    pub primary_release_date_gte: Option<String>,
    pub primary_release_date_lte: Option<String>,
    pub release_date_gte: Option<String>,
    pub release_date_lte: Option<String>,
    pub with_release_type: Option<u32>,
    pub year: Option<u32>,
    pub vote_count_gte: Option<u32>,
    pub vote_count_lte: Option<u32>,
    pub vote_average_gte: Option<f32>,
    pub vote_average_lte: Option<f32>,
    pub with_cast: Option<String>,
    pub with_crew: Option<String>,
    pub with_people: Option<String>,
    pub with_companies: Option<String>,
    pub with_genres: Option<String>,
    pub without_genres: Option<String>,
    pub with_keywords: Option<String>,
    pub without_keywords: Option<String>,
    pub with_runtime_gte: Option<u32>,
    pub with_runtime_lte: Option<u32>,
    pub with_original_language: Option<String>,
    pub with_watch_providers: Option<String>,
    pub watch_region: Option<String>,
    pub with_watch_monetization_types: Option<String>,
    pub without_companies: Option<String>,
}

impl Into<DiscoverMovieParams> for usize {
    fn into(self) -> DiscoverMovieParams {
        DiscoverMovieParams {
            page: Some(self as u32),
            ..Default::default()
        }
    }
}

impl Into<QueryParams> for DiscoverMovieParams {
    fn into(self) -> QueryParams {
        struct_to_map(&self).into()
    }
}

impl TmdbApi {
    pub async fn discover_movie(
        self: &TmdbApi,
        params: DiscoverMovieParams,
    ) -> Result<DiscoverMovieResponse, String> {
        let query_params: QueryParams = params.into();
        let req = self.to_get_request("/3/discover/movie", query_params);

        let sent = http::client::send(req).await;

        let response = match sent {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        match serde_json::from_str(&response.body) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                let err = format!("Error parsing response: {} {}", e, response.body);
                Err(err)
            }
        }
    }
}
