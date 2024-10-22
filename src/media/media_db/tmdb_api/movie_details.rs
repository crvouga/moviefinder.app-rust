// https://developer.themoviedb.org/reference/discover-movie
use serde::{Deserialize, Serialize};

use crate::{
    core::http,
    media::{genre::genre_id::GenreId, media::Media, media_id::MediaId, media_type::MediaType},
};

use super::{
    config::{to_backdrop_image_set, to_poster_image_set, TmdbConfig},
    to_get_request, Config,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BelongsToCollection {
    pub id: i64,
    pub name: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionCompany {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpokenLanguage {
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
            media_id: MediaId::new(result.id.to_string()),
            media_backdrop: to_backdrop_image_set(
                &config,
                result.backdrop_path.unwrap_or("".to_string()).as_str(),
            ),
            media_description: result.overview.unwrap_or("".to_string()),
            media_genre_ids: result
                .genres
                .unwrap_or(vec![])
                .iter()
                .map(|genre| GenreId::new(genre.id.to_string()))
                .collect(),
            media_popularity: result.popularity.unwrap_or(0.0),
            media_poster: to_poster_image_set(
                &config,
                result.poster_path.unwrap_or("".to_string()).as_str(),
            ),
            media_title: result.title.unwrap_or("".to_string()),
            media_type: MediaType::Movie,
        }
    }
}

pub async fn send(config: &Config, movie_id: &str) -> Result<MovieDetails, String> {
    let req = to_get_request(config, &format!("/3/movie/{}", movie_id));

    let response = http::client::send(req).await.map_err(|e| e.to_string())?;

    let parsed: MovieDetails = serde_json::from_str(&response.body)
        .map_err(|e| format!("Error parsing response: {} {}", e, response.body))?;

    Ok(parsed)
}
