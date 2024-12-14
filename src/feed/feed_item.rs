use crate::{
    core::{
        html::{button, div, Elem},
        ui::image::Image,
    },
    media::{self, interaction::interaction_form, media_::Media},
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

    pub fn view_slide_content(self: &FeedItem) -> Elem {
        match self {
            FeedItem::Media {
                media,
                feed_index: _,
            } => div()
                .class("w-full h-full max-h-full overflow-hidden flex flex-col")
                .child(
                    button()
                        .class("w-full flex-1 overflow-hidden active:opacity-80")
                        .data_on(|b| {
                            b.click().push_then_sse(
                                &media::details::route::Route::MediaDetailsScreen {
                                    media_id: media.id.clone(),
                                }
                                .url(),
                            )
                        })
                        .aria_label("open media details")
                        .child(
                            Image::new()
                                .view()
                                .src(media.poster.to_highest_res())
                                .class("w-full h-full object-cover")
                                .width("100%")
                                .height("100%")
                                .alt(media.title.as_str()),
                        ),
                )
                .child(div().class("shrink-0 w-full border-t").child(
                    interaction_form::respond::view_form_bottom_bar_load(&media.id),
                )),
        }
    }
}

impl From<(Media, usize)> for FeedItem {
    fn from((media, feed_index): (Media, usize)) -> Self {
        FeedItem::Media { media, feed_index }
    }
}
