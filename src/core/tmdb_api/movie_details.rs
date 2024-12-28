// https://developer.themoviedb.org/reference/discover-movie
use serde::{Deserialize, Serialize};

use crate::{
    core::url::query_params::QueryParams,
    media::{genre::genre_id::GenreId, media_::Media, media_id::MediaId, media_type::MediaType},
};

use super::{config::TmdbConfig, TmdbApi};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BelongsToCollection {
    pub id: i64,
    pub name: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductionCompany {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpokenLanguage {
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDetails {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<BelongsToCollection>,
    pub budget: Option<i64>,
    pub genres: Option<Vec<Genre>>,
    pub homepage: Option<String>,
    pub id: i64,
    pub imdb_id: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub production_companies: Option<Vec<ProductionCompany>>,
    pub production_countries: Option<Vec<ProductionCountry>>,
    pub release_date: Option<String>,
    pub revenue: Option<i64>,
    pub runtime: Option<i64>,
    pub spoken_languages: Option<Vec<SpokenLanguage>>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
}

impl From<(&TmdbConfig, MovieDetails)> for Media {
    fn from((config, result): (&TmdbConfig, MovieDetails)) -> Self {
        Media {
            id: MediaId::new(result.id.to_string()),
            backdrop: config
                .to_backdrop_image_set(result.backdrop_path.unwrap_or("".to_string()).as_str()),
            description: result.overview.unwrap_or("".to_string()),
            genre_ids: result
                .genres
                .unwrap_or(vec![])
                .iter()
                .map(|genre| GenreId::new(genre.id.to_string()))
                .collect(),
            popularity: result.popularity.unwrap_or(0.0),
            poster: config
                .to_poster_image_set(result.poster_path.unwrap_or("".to_string()).as_str()),
            title: result.title.unwrap_or("".to_string()),
            media_type: MediaType::Movie,
        }
    }
}

impl TmdbApi {
    pub async fn movie_details(self: &TmdbApi, movie_id: &str) -> Result<MovieDetails, String> {
        let req = self.to_get_request(&format!("/3/movie/{}", movie_id), QueryParams::default());

        let response = self
            .http_client
            .send(req)
            .await
            .map_err(|e| e.to_string())?;

        let parsed: MovieDetails = serde_json::from_str(&response.clone().to_body_string())
            .map_err(|e| {
                format!(
                    "Error parsing response: {} {}",
                    e,
                    response.to_body_string()
                )
            })?;

        Ok(parsed)
    }
}
