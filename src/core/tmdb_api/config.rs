// https://developer.themoviedb.org/reference/configuration-details
use super::TmdbApi;
use crate::core::{http::query_params::QueryParams, image_set::ImageSet};
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

impl TmdbApi {
    pub async fn config(self: &TmdbApi) -> Result<TmdbConfig, String> {
        let req = self.to_get_request("/3/configuration", QueryParams::default());

        let sent = self.http_client.send(req).await;

        match sent {
            Ok(response) => {
                let body = response.to_body_string();
                let parsed = serde_json::from_str(&body);
                match parsed {
                    Ok(parsed) => Ok(parsed),
                    Err(e) => Err(format!("Error parsing response: {} {}", e, body)),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

impl TmdbConfig {
    pub fn to_poster_image_set(self: &TmdbConfig, poster_path: &str) -> ImageSet {
        self.to_image_set(self.to_poster_sizes(), poster_path)
    }

    pub fn to_backdrop_image_set(self: &TmdbConfig, backdrop_path: &str) -> ImageSet {
        self.to_image_set(self.to_backdrop_sizes(), backdrop_path)
    }

    fn to_poster_sizes(self: &TmdbConfig) -> Vec<String> {
        self.images.as_ref().map_or(vec![], |images| {
            images.poster_sizes.as_ref().unwrap_or(&vec![]).clone()
        })
    }

    fn to_backdrop_sizes(self: &TmdbConfig) -> Vec<String> {
        self.images.as_ref().map_or(vec![], |images| {
            images.backdrop_sizes.as_ref().unwrap_or(&vec![]).clone()
        })
    }

    fn to_image_set(self: &TmdbConfig, sizes: Vec<String>, path: &str) -> ImageSet {
        let base_url = self.to_base_url();

        let lowest_to_highest = sizes
            .iter()
            .map(|size| format!("{}{}{}", base_url, size, path));

        ImageSet::new(lowest_to_highest.collect())
    }

    fn to_base_url(self: &TmdbConfig) -> String {
        self.images
            .as_ref()
            .map_or("".to_string(), |images| images.secure_base_url.clone())
    }
}
