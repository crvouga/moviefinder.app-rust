// https://developer.themoviedb.org/reference/discover-movie
use crate::{
    core::{unstructed_data::UnstructuredData, url::query_params::QueryParams},
    media::genre::{genre::Genre, genre_id::GenreId},
};

use super::TmdbApi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieGenreResult {
    id: Option<usize>,
    name: Option<String>,
}

impl From<MovieGenreResult> for Genre {
    fn from(movie_genre_result: MovieGenreResult) -> Self {
        Genre {
            id: GenreId::new(movie_genre_result.id.unwrap_or(0).to_string()),
            name: movie_genre_result.name.unwrap_or("".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieGenreResponse {
    pub genres: Option<Vec<Option<MovieGenreResult>>>,
}

impl TmdbApi {
    pub async fn movie_genre(self: &TmdbApi) -> Result<MovieGenreResponse, String> {
        let req = self.to_get_request("/3/genre/movie/list", QueryParams::empty());

        let sent = self.http_client.send(req).await;

        let response = sent.map_err(|err| err.to_string())?;

        let parsed = serde_json::from_str::<MovieGenreResponse>(&response.clone().to_body_string())
            .map_err(|err| {
                format!(
                    "Error parsing response: {} {}",
                    err,
                    response.to_body_string()
                )
            })?;

        Ok(parsed)
    }
}
