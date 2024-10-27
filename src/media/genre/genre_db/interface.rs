use async_trait::async_trait;

use crate::media::genre::genre::Genre;

#[async_trait]
pub trait GenreDb: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Genre>, String>;
}
