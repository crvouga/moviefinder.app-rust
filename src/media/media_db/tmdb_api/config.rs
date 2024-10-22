// https://developer.themoviedb.org/reference/configuration-details
use super::{to_get_request, Config};
use crate::{core::http, core::image_set::ImageSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TmdbConfig {
    pub images: Option<TmdbConfigImages>,
    pub change_keys: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmdbConfigImages {
    pub base_url: String,
    pub secure_base_url: String,
    pub backdrop_sizes: Option<Vec<String>>,
    pub logo_sizes: Option<Vec<String>>,
    pub poster_sizes: Option<Vec<String>>,
    pub profile_sizes: Option<Vec<String>>,
    pub still_sizes: Option<Vec<String>>,
}

pub async fn load(config: &Config) -> Result<TmdbConfig, String> {
    let req = to_get_request(config, "/3/configuration");

    let sent = http::client::send(req).await;

    match sent {
        Ok(response) => {
            let body = response.body;
            let parsed = serde_json::from_str(&body);
            match parsed {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(format!("Error parsing response: {} {}", e, body)),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn to_poster_image_set(config: &TmdbConfig, poster_path: &str) -> ImageSet {
    to_image_set(&config, to_poster_sizes(&config), poster_path)
}

pub fn to_backdrop_image_set(config: &TmdbConfig, backdrop_path: &str) -> ImageSet {
    to_image_set(&config, to_backdrop_sizes(&config), backdrop_path)
}

fn to_poster_sizes(config: &TmdbConfig) -> Vec<String> {
    config.images.as_ref().map_or(vec![], |images| {
        images.poster_sizes.as_ref().unwrap_or(&vec![]).clone()
    })
}

fn to_backdrop_sizes(config: &TmdbConfig) -> Vec<String> {
    config.images.as_ref().map_or(vec![], |images| {
        images.backdrop_sizes.as_ref().unwrap_or(&vec![]).clone()
    })
}

fn to_image_set(config: &TmdbConfig, sizes: Vec<String>, path: &str) -> ImageSet {
    let base_url = to_base_url(&config);

    let lowest_to_highest = sizes
        .iter()
        .map(|size| format!("{}/{}{}", base_url, size, path));

    ImageSet::new(lowest_to_highest.collect())
}

fn to_base_url(config: &TmdbConfig) -> String {
    config
        .images
        .as_ref()
        .map_or("".to_string(), |images| images.base_url.clone())
}
