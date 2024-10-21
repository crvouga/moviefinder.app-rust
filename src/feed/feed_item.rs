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

impl From<&FeedItem> for Elem {
    fn from(feed_item: &FeedItem) -> Self {
        match feed_item {
            FeedItem::Media { media, feed_index } => ui::swiper::slide(
                &[class(
                    "w-full h-full flex flex-col items-center justify-center",
                )],
                &[text(&format!("{}: {}", feed_index, media.media_title))],
            ),
        }
    }
}
