use crate::media::core::Media;

#[derive(Debug, Clone)]
pub enum FeedItem {
    Media { media: Media, feed_index: usize },
}

impl FeedItem {
    pub fn to_feed_index(self: &FeedItem) -> usize {
        match self {
            FeedItem::Media { feed_index, .. } => *feed_index,
        }
    }
}

impl From<(Media, usize)> for FeedItem {
    fn from(value: (Media, usize)) -> Self {
        FeedItem::Media {
            media: value.0,
            feed_index: value.1,
        }
    }
}
