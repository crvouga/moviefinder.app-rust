use serde::{Deserialize, Serialize};

use super::{
    interaction_action::InteractionAction, interaction_id::MediaInteractionId,
    interaction_name::InteractionName,
};
use crate::{core::posix::Posix, media::media_id::MediaId, user::user_id::UserId};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MediaInteraction {
    pub id: MediaInteractionId,
    pub user_id: UserId,
    pub media_id: MediaId,
    pub created_at_posix: Posix,
    pub interaction_name: InteractionName,
    pub interaction_action: InteractionAction,
}

impl MediaInteraction {
    #[allow(dead_code)]
    pub fn random() -> Self {
        Self {
            id: MediaInteractionId::default(),
            user_id: UserId::default(),
            media_id: MediaId::default(),
            created_at_posix: Posix::default(),
            interaction_name: InteractionName::random(),
            interaction_action: InteractionAction::random(),
        }
    }
}
