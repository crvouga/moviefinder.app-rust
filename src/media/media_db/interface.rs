use crate::core::pagination::Paginated;
use crate::core::query::Query;
use crate::media::core::Media;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MediaField {
    #[default]
    MediaId,
    GenreId,
}

pub type MediaQuery = Query<MediaField>;

#[async_trait]
pub trait MediaDb: Send + Sync {
    async fn query(&self, query: MediaQuery) -> Result<Paginated<Media>, String>;
}
