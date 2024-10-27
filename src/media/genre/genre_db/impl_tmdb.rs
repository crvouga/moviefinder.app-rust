use std::sync::Arc;

use async_trait::async_trait;

use crate::media::{genre::genre::Genre, tmdb_api::TmdbApi};

use super::interface::GenreDb;

pub struct ImplTmdb {
    tmdb_api: Arc<TmdbApi>,
}

impl ImplTmdb {
    pub fn new(tmdb_api: Arc<TmdbApi>) -> ImplTmdb {
        ImplTmdb { tmdb_api }
    }
}

#[async_trait]
impl GenreDb for ImplTmdb {
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
