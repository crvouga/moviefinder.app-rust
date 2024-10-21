pub mod impl_random;
pub mod impl_tmdb_movie;
mod mod_test;
mod tmdb_api;

use async_trait::async_trait;

use crate::core::pagination::Paginated;
use crate::core::query::Query;
use crate::media::media::Media;

pub enum Field {
    MediaId,
}

#[async_trait]
pub trait MediaDb: Send + Sync {
    async fn query(&self, query: &Query<Field>) -> Result<Paginated<Media>, String>;
}
