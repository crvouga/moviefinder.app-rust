use crate::{
    core::unit_of_work::UnitOfWork, media::interaction::interaction_::MediaInteraction,
    media::media_id::MediaId, user::user_id::UserId,
};
use async_trait::async_trait;

#[async_trait]
pub trait MediaInteractionDb: Send + Sync {
    async fn list_by_user_media(
        &self,
        user_id: &UserId,
        media_ids: &Vec<&MediaId>,
    ) -> Result<Vec<MediaInteraction>, std::io::Error>;
    async fn put(
        &self,
        uow: UnitOfWork,
        interaction: &MediaInteraction,
    ) -> Result<(), std::io::Error>;
}
