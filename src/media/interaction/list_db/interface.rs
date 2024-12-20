use crate::{list::list::List, user::user_id::UserId};
use async_trait::async_trait;

#[async_trait]
pub trait ListDb: Send + Sync {
    async fn find_by_user_id(&self, user_id: UserId) -> Result<Vec<List>, std::io::Error>;
}
