use crate::{
    core::{
        html::{div, Elem},
        ui::image::Image,
    },
    media::{self, interaction::interaction_form, media_::Media, media_id::MediaId},
    ui::route::AppRoute,
};

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

    pub fn to_media_id(self: &FeedItem) -> Option<MediaId> {
        match self {
            FeedItem::Media { media, .. } => Some(media.id.clone()),
        }
    }
}

impl From<(Media, usize)> for FeedItem {
    fn from((media, feed_index): (Media, usize)) -> Self {
        FeedItem::Media { media, feed_index }
    }
}
