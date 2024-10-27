use crate::core::pagination::Paginated;
use crate::core::query::Query;
use crate::media::core::Media;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum MediaField {
    MediaId,
    GenreId,
}

#[async_trait]
pub trait MediaDb: Send + Sync {
    async fn query(&self, query: Query<MediaField>) -> Result<Paginated<Media>, String>;
}
