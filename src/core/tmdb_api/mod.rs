use super::{
    http::method::Method,
    params::Params,
    url::{query_params::QueryParams, Url},
};
use crate::core::http::{client::HttpClient, form_data::FormData, request::Request};
use std::{collections::HashMap, sync::Arc};

pub mod config;
pub mod discover_movie;
pub mod movie_details;
pub mod movie_genre;
pub mod person;
pub mod person_popular;
pub mod person_search;

#[derive(Clone)]
pub struct TmdbApi {
    tmdb_api_read_access_token: String,
    http_client: Arc<HttpClient>,
}

impl TmdbApi {
    pub fn new(http_client: Arc<HttpClient>, tmdb_api_read_access_token: String) -> TmdbApi {
        TmdbApi {
            http_client,
            tmdb_api_read_access_token,
        }
    }
}

pub const TMDB_PAGE_SIZE: usize = 20;
pub const TMDB_HOST: &str = "api.themoviedb.org";
pub const TMDB_IMAGE_BASE_URL: &str = "https://image.tmdb.org";

impl TmdbApi {
    pub fn to_get_request(self: &TmdbApi, path: &str, query_params: QueryParams) -> Request {
        self.to_request(Method::Get, path, query_params)
    }

    pub fn to_request(
        self: &TmdbApi,
        method: Method,
        path: &str,
        query_params: QueryParams,
    ) -> Request {
        Request {
            headers: self.to_base_headers(),
            url: Url {
                host: TMDB_HOST.to_string(),
                path: path.to_string(),
                query_params,
            },
            method,
            body: vec![],
            cookies: HashMap::new(),
            form_data: FormData::empty(),
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
