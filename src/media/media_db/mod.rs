pub mod random;
mod tmdb_api;
pub mod tmdb_movie;

use async_trait::async_trait;

use crate::core::pagination::Paginated;
use crate::media::media::Media;

pub struct Query {
    pub limit: u32,
    pub offset: u32,
}

#[async_trait]
pub trait MediaDb: Send + Sync {
    async fn query(&self, query: &Query) -> Result<Paginated<Media>, String>;
}
