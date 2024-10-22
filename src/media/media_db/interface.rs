use crate::core::pagination::Paginated;
use crate::core::query::Query;
use crate::media::media::Media;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum Field {
    MediaId,
}

#[async_trait]
pub trait MediaDb: Send + Sync {
    async fn query(&self, query: Query<Field>) -> Result<Paginated<Media>, String>;
}
