use std::collections::HashMap;

use crate::core::http;

pub mod config;
pub mod discover_movie;
pub mod movie_details;

#[derive(Debug)]
pub struct Config {
    tmdb_api_read_access_token: String,
}

impl Config {
    pub fn new(tmdb_api_read_access_token: String) -> Config {
        Config {
            tmdb_api_read_access_token,
        }
    }
}

pub const HOST: &str = "api.themoviedb.org";

pub fn to_request(config: &Config, method: &str, path: &str) -> http::Request {
    http::Request {
        headers: to_base_headers(config),
        host: HOST.to_string(),
        method: method.to_string(),
        path: path.to_string(),
        body: "".to_string(),
    }
}

pub fn to_get_request(config: &Config, path: &str) -> http::Request {
    to_request(config, "GET", path)
}

// pub const PAGE_SIZE: u32 = 20;

pub fn to_base_headers(config: &Config) -> HashMap<String, String> {
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
        format!("Bearer {}", config.tmdb_api_read_access_token),
    );
    headers
}
