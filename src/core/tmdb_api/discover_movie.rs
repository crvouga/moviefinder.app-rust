// https://developer.themoviedb.org/reference/discover-movie
use crate::core::{
    dynamic_data::DynamicData, struct_ext::struct_to_map, url::query_params::QueryParams,
};

use super::TmdbApi;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoverMovieResponse {
    pub page: Option<usize>,
    pub results: Option<Vec<DiscoverMovieResult>>,
    pub total_pages: Option<usize>,
    pub total_results: Option<usize>,
}

pub const TMDB_AND_OP: &str = ",";
pub const TMDB_OR_OP: &str = "|";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    pub page: Option<usize>,
    pub primary_release_year: Option<usize>,
    pub primary_release_date_gte: Option<String>,
    pub primary_release_date_lte: Option<String>,
    pub release_date_gte: Option<String>,
    pub release_date_lte: Option<String>,
    pub with_release_type: Option<usize>,
    pub year: Option<usize>,
    pub vote_count_gte: Option<usize>,
    pub vote_count_lte: Option<usize>,
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
    pub with_runtime_gte: Option<usize>,
    pub with_runtime_lte: Option<usize>,
    pub with_original_language: Option<String>,
    pub with_watch_providers: Option<String>,
    pub watch_region: Option<String>,
    pub with_watch_monetization_types: Option<String>,
    pub without_companies: Option<String>,
}

impl Into<QueryParams> for DiscoverMovieParams {
    fn into(self) -> QueryParams {
        QueryParams::from_hash_map(struct_to_map(&self))
    }
}

impl TmdbApi {
    pub async fn discover_movie(
        &self,
        params: DiscoverMovieParams,
    ) -> Result<DiscoverMovieResponse, String> {
        let query_params: QueryParams = params.into();

        let req = self.to_get_request("/3/discover/movie", query_params);

        let sent = self.http_client.send(req).await;

        let response = match sent {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        match serde_json::from_str(&response.clone().to_body_string()) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                let err = format!(
                    "Error parsing response: {} {}",
                    e,
                    response.to_body_string()
                );
                Err(err)
            }
        }
    }
}
