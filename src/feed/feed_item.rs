use crate::media::core::Media;

pub enum FeedItem {
    Media { media: Media, feed_index: usize },
}

impl From<(Media, usize)> for FeedItem {
    fn from(value: (Media, usize)) -> Self {
        FeedItem::Media {
            media: value.0,
            feed_index: value.1,
        }
    }
}
