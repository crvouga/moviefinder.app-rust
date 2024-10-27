use crate::core::http::{query_params::QueryParams, request::HttpRequest};
use std::collections::HashMap;

pub mod config;
pub mod discover_movie;
pub mod movie_details;
pub mod movie_genre;

#[derive(Debug)]
pub struct TmdbApi {
    tmdb_api_read_access_token: String,
}

impl TmdbApi {
    pub fn new(tmdb_api_read_access_token: String) -> TmdbApi {
        TmdbApi {
            tmdb_api_read_access_token,
        }
    }
}

pub const TMDB_PAGE_SIZE: usize = 20;
pub const TMDB_HOST: &str = "api.themoviedb.org";

impl TmdbApi {
    pub fn to_get_request(self: &TmdbApi, path: &str, query_params: QueryParams) -> HttpRequest {
        self.to_request("GET", path, query_params)
    }

    pub fn to_request(
        self: &TmdbApi,
        method: &str,
        path: &str,
        query_params: QueryParams,
    ) -> HttpRequest {
        HttpRequest {
            headers: self.to_base_headers(),
            host: TMDB_HOST.to_string(),
            method: method.to_string(),
            path: path.to_string(),
            body: "".to_string(),
            cookies: HashMap::new(),
            form_data: HashMap::new(),
            query_params,
        }
    }

    pub fn to_base_headers(self: &TmdbApi) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string().to_ascii_lowercase(),
            "application/json".to_string(),
        );
        headers.insert(
            "Accept".to_string().to_ascii_lowercase(),
            "application/json".to_string(),
        );
        headers.insert(
            "Authorization".to_string().to_ascii_lowercase(),
            format!("Bearer {}", self.tmdb_api_read_access_token),
        );
        headers
    }
}
