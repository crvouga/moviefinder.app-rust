use std::sync::Arc;

use async_trait::async_trait;

use crate::{core::tmdb_api::TmdbApi, media::genre::genre::Genre};

use super::interface::MediaGenreDb;

pub struct Tmdb {
    tmdb_api: Arc<TmdbApi>,
}

impl Tmdb {
    pub fn new(tmdb_api: Arc<TmdbApi>) -> Tmdb {
        Tmdb { tmdb_api }
    }
}

#[async_trait]
impl MediaGenreDb for Tmdb {
    async fn get_all(&self) -> Result<Vec<Genre>, String> {
        let movie_genres = self
            .tmdb_api
            .movie_genre()
            .await
            .map_err(|e| e.to_string())?;

        let genres: Vec<Genre> = movie_genres
            .genres
            .unwrap_or(vec![])
            .into_iter()
            .filter_map(|genre| genre)
            .map(|genre| genre.into())
            .collect();

        Ok(genres)
    }
}
