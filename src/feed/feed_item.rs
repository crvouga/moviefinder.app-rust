use crate::html::*;
use crate::media::media::Media;
use crate::ui;

pub enum FeedItem {
    Media { media: Media, feed_index: i32 },
}

impl From<(Media, i32)> for FeedItem {
    fn from(value: (Media, i32)) -> Self {
        FeedItem::Media {
            media: value.0,
            feed_index: value.1,
        }
    }
}
