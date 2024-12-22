use async_trait::async_trait;

use crate::{media::interaction::list::list_::MediaInteractionList, user::user_id::UserId};

#[async_trait]
pub trait MediaInteractionListDb: Send + Sync {
    async fn find_by_user_id(
        &self,
        user_id: UserId,
    ) -> Result<Vec<MediaInteractionList>, std::io::Error>;
}
