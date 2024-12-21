use crate::{media::interaction::list::MediaInteractionList, user::user_id::UserId};
use async_trait::async_trait;

#[async_trait]
pub trait MediaInteractionListDb: Send + Sync {
    async fn find_by_user_id(
        &self,
        user_id: UserId,
    ) -> Result<Vec<MediaInteractionList>, std::io::Error>;
}
