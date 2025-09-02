use crate::core::pagination::Paginated;
use crate::core::query::Query;
use crate::media::media_::Media;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MediaQueryField {
    #[default]
    MediaId,
    GenreId,
    PersonId,
}

pub type MediaQuery = Query<MediaQueryField>;

#[async_trait]
pub trait MediaDb: Send + Sync {
    async fn query(&self, query: MediaQuery)
        -> Result<Paginated<Media>, crate::core::error::CoreError>;
}
