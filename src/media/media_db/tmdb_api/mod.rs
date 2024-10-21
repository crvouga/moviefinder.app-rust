use std::collections::HashMap;

pub mod config;
pub mod discover_movie;

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

// pub const PAGE_SIZE: u32 = 20;

pub fn to_base_headers(config: &Config) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Accept".to_string(), "application/json".to_string());
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", config.tmdb_api_read_access_token),
    );
    headers
}
