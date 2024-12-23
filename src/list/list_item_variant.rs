use crate::media::media_id::MediaId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaListItemVariant {
    Media(MediaId),
}

impl Default for MediaListItemVariant {
    fn default() -> Self {
        MediaListItemVariant::Media(MediaId::default())
    }
}
