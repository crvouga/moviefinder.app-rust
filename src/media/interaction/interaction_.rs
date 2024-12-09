use serde::{Deserialize, Serialize};

use super::{interaction_action::InteractionAction, interaction_name::InteractionName};
use crate::{core::posix::Posix, media::media_id::MediaId, user::user_id::UserId};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MediaInteraction {
    pub user_id: UserId,
    pub media_id: MediaId,
    pub created_at_posix: Posix,
    pub interaction_name: InteractionName,
    pub interaction_action: InteractionAction,
}
