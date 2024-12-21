use serde::{Deserialize, Serialize};

use crate::media::interaction::{
    interaction_action::InteractionAction, interaction_name::InteractionName,
};

use super::MediaInteraction;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaInteractionPostgresRow {
    pub id: Option<String>,
    pub media_id: Option<String>,
    pub user_id: Option<String>,
    pub interaction_name: Option<String>,
    pub interaction_action: Option<String>,
    pub created_at_posix: Option<i64>,
    pub updated_at_posix: Option<i64>,
    pub deleted_at_posix: Option<i64>,
}

impl MediaInteractionPostgresRow {
    pub fn from_json(value: serde_json::Value) -> Result<Self, std::io::Error> {
        serde_json::from_value(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn to_media_interaction(self) -> Option<MediaInteraction> {
        let name = InteractionName::from_string(self.interaction_name.unwrap_or_default())?;

        let action = InteractionAction::from_string(self.interaction_action.unwrap_or_default())?;

        let interaction = MediaInteraction {
            interaction_name: name,
            interaction_action: action,
            id: self.id.unwrap_or_default().into(),
            media_id: self.media_id.unwrap_or_default().into(),
            user_id: self.user_id.unwrap_or_default().into(),
            created_at_posix: self.created_at_posix.unwrap_or_default().into(),
        };

        Some(interaction)
    }
}
