use super::{list_id::ListId, list_item_id::ListItemId};
use crate::{
    core::posix::Posix,
    media::{interaction::interaction_::MediaInteraction, media_id::MediaId},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItem {
    pub id: ListItemId,
    pub list_id: ListId,
    pub variant: ListItemVariant,
    pub created_at_posix: Posix,
}

impl From<(ListId, MediaInteraction)> for ListItem {
    fn from((list_id, interaction): (ListId, MediaInteraction)) -> Self {
        Self {
            id: ListItemId::from_string(interaction.id.as_str()),
            list_id,
            variant: ListItemVariant::Media(interaction.media_id),
            created_at_posix: interaction.created_at_posix,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemVariant {
    Media(MediaId),
}

impl Default for ListItemVariant {
    fn default() -> Self {
        ListItemVariant::Media(MediaId::default())
    }
}
