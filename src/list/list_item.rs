use super::{
    list_id::MediaListId, list_item_id::MediaListItemId, list_item_variant::MediaListItemVariant,
};
use crate::{core::posix::Posix, media::interaction::interaction_::MediaInteraction};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MediaListItem {
    pub id: MediaListItemId,
    pub list_id: MediaListId,
    pub variant: MediaListItemVariant,
    pub created_at_posix: Posix,
}

impl From<(MediaListId, MediaInteraction)> for MediaListItem {
    fn from((list_id, interaction): (MediaListId, MediaInteraction)) -> Self {
        Self {
            id: MediaListItemId::from_string(interaction.id.as_str()),
            list_id,
            variant: MediaListItemVariant::Media(interaction.media_id),
            created_at_posix: interaction.created_at_posix,
        }
    }
}
