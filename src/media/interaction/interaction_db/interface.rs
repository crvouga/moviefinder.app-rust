use crate::{
    core::unit_of_work::UnitOfWork,
    media::interaction::{interaction_::MediaInteraction, interaction_name::InteractionName},
    media::media_id::MediaId,
    user::user_id::UserId,
};
use async_trait::async_trait;

#[async_trait]
pub trait MediaInteractionDb: Send + Sync {
    async fn find_by_user_id_and_media_ids(
        &self,
        user_id: &UserId,
        media_ids: &Vec<&MediaId>,
    ) -> Result<Vec<MediaInteraction>, crate::core::error::CoreError>;
    #[allow(dead_code)]
    async fn find_by_user_id_and_interaction_name(
        &self,
        user_id: &UserId,
        interaction_name: &InteractionName,
    ) -> Result<Vec<MediaInteraction>, crate::core::error::CoreError>;
    async fn put(
        &self,
        u: UnitOfWork,
        interaction: &MediaInteraction,
    ) -> Result<(), crate::core::error::CoreError>;
}
