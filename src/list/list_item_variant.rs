use crate::media::media_id::MediaId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemVariant {
    Media(MediaId),
}

impl Default for ListItemVariant {
    fn default() -> Self {
        ListItemVariant::Media(MediaId::default())
    }
}
