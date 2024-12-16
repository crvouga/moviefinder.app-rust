use crate::{
    core::html::{div, img, Elem},
    media::{self, interaction::interaction_form, media_::Media, media_id::MediaId},
    ui::route::Routable,
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

    pub fn view_slide_content(self: &FeedItem) -> Elem {
        match self {
            FeedItem::Media {
                media,
                feed_index: _,
            } => div()
                .class("w-full h-full max-h-full flex flex-col divide-y overflow-hidden")
                .child(
                    img()
                        .class("w-full flex-1 overflow-hidden object-cover active:opacity-80 z-0 pointer-cursor")
                        .tab_index(0)
                        .role_button()
                        .data_on(|b| {
                            b.click().push_then_sse(
                                &media::details::route::Route::MediaDetailsScreen {
                                    media_id: media.id.clone(),
                                }
                                .url(),
                            )
                        })
                        .aria_label("open media details")
                        .src(media.poster.to_highest_res())
                        .alt(media.title.as_str()),
                )
                .child(interaction_form::respond::view_interaction_form(
                    &media.id, None,
                )),
        }
    }
}

impl From<(Media, usize)> for FeedItem {
    fn from((media, feed_index): (Media, usize)) -> Self {
        FeedItem::Media { media, feed_index }
    }
}
