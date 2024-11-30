use super::{feed_::Feed, feed_id::FeedId, feed_item::FeedItem, feed_tags, route::Route};
use crate::{
    core::{
        html::*,
        http::{response_writer::ResponseWriter, server_sent_event::sse},
        session::session_id::SessionId,
        ui::{self, chip::ChipSize, icon, image::Image},
    },
    ctx::Ctx,
    media::{self, media_id::MediaId},
    req::Req,
    route,
    ui::{bottom_bar, top_bar},
};

pub const LIMIT: usize = 3;

pub struct ViewModel {
    pub feed: Feed,
    pub initial_feed_items: Vec<FeedItem>,
}

pub async fn respond_index(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
    feed_id: &FeedId,
) -> Result<(), std::io::Error> {
    sse()
        .event_merge_fragments()
        .data_fragments(view_index_loading(&feed_id))
        .send(w)
        .await?;

    let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

    put_feed(ctx, &r.session_id, &feed).await;

    let initial_feed_items = get_feed_items(ctx, &feed).await.unwrap_or_default();

    let model: ViewModel = ViewModel {
        feed: feed.clone(),
        initial_feed_items,
    };

    sse()
        .event_merge_fragments()
        // .data_merge_mode_outer()
        .data_fragments(view_index(&model))
        .send(w)
        .await?;

    Ok(())
}

pub async fn put_feed(ctx: &Ctx, session_id: &SessionId, feed: &Feed) {
    let put_feed_fut = ctx.feed_db.put(feed.clone());
    let put_session_mapping_fut = ctx
        .feed_session_mapping_db
        .put(session_id.clone(), feed.feed_id.clone());

    let _results = tokio::join!(put_feed_fut, put_session_mapping_fut);
}

pub async fn get_feed_items(ctx: &Ctx, feed: &Feed) -> Result<Vec<FeedItem>, String> {
    let query = feed.to_media_query(LIMIT);

    let queried = ctx.media_db.query(query).await;

    match queried {
        Err(err) => Err(err),

        Ok(paginated) => {
            let feed_items = paginated
                .items
                .into_iter()
                .enumerate()
                .map(|(index, media)| FeedItem::from((media, index + feed.start_index)))
                .collect::<Vec<FeedItem>>();

            Ok(feed_items)
        }
    }
}

fn view_top_bar_root() -> Elem {
    top_bar::view_root()
        .button()
        .class("relative")
        .aria_label("open controls")
        .id("top-bar")
}

fn view_top_bar_link_root(feed_id: &FeedId) -> Elem {
    view_top_bar_root().data_on_click_push_then_get(
        &route::Route::Feed(Route::Controls(feed_tags::route::Route::Index {
            feed_id: feed_id.clone(),
        }))
        .encode(),
    )
}

fn view_top_bar(model: &ViewModel) -> Elem {
    view_top_bar_link_root(&model.feed.feed_id)
        .child(view_tags(&model))
        .child(view_open_controls_button())
}

fn view_slide_content_empty() -> Elem {
    Image::new()
        .view()
        .src(" ")
        .class("w-full h-full object-cover")
}

fn view_root() -> Elem {
    div()
        .id_root()
        .class("w-full flex-1 flex items-center justify-center flex-col overflow-hidden")
        .data_store("{signalFeedIndex: 0, signalTrue: true}")
}

fn view_index_loading(feed_id: &FeedId) -> Elem {
    view_root()
        .child(view_top_bar_link_root(&feed_id).child(view_open_controls_button()))
        .child(view_slide_content_empty())
        .child(view_bottom_bar())
}

pub fn view_default_loading() -> Elem {
    view_root()
        .child(view_top_bar_root().child(view_open_controls_button()))
        .child(view_slide_content_empty())
        .child(view_bottom_bar())
}

fn view_open_controls_button() -> Elem {
    div()
        .class("absolute top-0 right-0 h-full flex items-center justify-center")
        .child(div().class("w-16 h-full from-transparent to-black bg-gradient-to-r"))
        .child(
            div()
                .class("size-16 bg-black flex items-center justify-center")
                .child(ui::icon::pencil("size-6")),
        )
}

fn view_tags(model: &ViewModel) -> Elem {
    div()
        .id("tags")
        .class("flex flex-row gap-2 p-4 flex-1 overflow-hidden")
        .children(
            model
                .feed
                .tags
                .iter()
                .map(|tag| {
                    tag.chip()
                        .signal_checked("$signalTrue")
                        .disabled(true)
                        .id(&tag.encode().to_lowercase())
                        .size(ChipSize::Small)
                        .view()
                })
                .collect::<Vec<Elem>>(),
        )
}

fn view_index(model: &ViewModel) -> Elem {
    view_root()
        .child(view_top_bar(&model))
        .child(view_swiper(&model))
        .child(view_bottom_bar())
}

fn view_bottom_bar() -> Elem {
    bottom_bar::view(bottom_bar::Active::Home, "")
}

fn view_swiper(model: &ViewModel) -> Elem {
    if model.initial_feed_items.len() == 0 {
        return view_swiper_empty();
    }
    ui::swiper::container()
        .swiper_direction_vertical()
        .swiper_slides_per_view("1")
        .class("flex-1 flex flex-col w-full items-center justify-center overflow-hidden")
        .data_on("swiperslidechange", "$signalFeedIndex = parseInt(evt?.detail?.[0]?.slides?.[event?.detail?.[0]?.activeIndex]?.getAttribute?.('data-feed-index'), 10)")
        .data_on_then_patch("swiperslidechange",route::Route::Feed(Route::ChangedSlide { feed_id: model.feed.feed_id.clone() }).encode().as_str())
        .child(view_slides(&model.feed.feed_id, &model.initial_feed_items))
}

fn view_swiper_empty() -> Elem {
    div()
        .class("w-full h-full flex items-center justify-center flex-col gap-4")
        .child(icon::magnifying_glass("size-16"))
        .child(
            div()
                .class("text-2xl font-bold w-full text-center")
                .child_text("No results found"),
        )
}

fn view_slides(feed_id: &FeedId, feed_items: &[FeedItem]) -> Elem {
    frag()
        .children(feed_items.iter().map(view_slide).collect::<Vec<Elem>>())
        .child(
            feed_items
                .iter()
                .last()
                .map(|last_feed_item| {
                    view_slide_content_bottom(feed_id, last_feed_item.to_feed_index() + 1)
                })
                .unwrap_or(frag()),
        )
}

fn view_slide_root() -> Elem {
    ui::swiper::slide()
        .class("w-full h-full flex flex-col items-center justify-center cursor-pointer relative")
}

impl Elem {
    pub fn data_feed_index(self, feed_index: usize) -> Self {
        self.attr("data-feed-index", &feed_index.to_string())
    }
}

pub const BOTTOM_ID: &str = "load-more";

fn view_slide_content_bottom(feed_id: &FeedId, bottom_feed_index: usize) -> Elem {
    view_slide_root()
        .id(BOTTOM_ID)
        .data_feed_index(bottom_feed_index)
        .data_intersects_get(
            &route::Route::Feed(Route::IntersectedBottom {
                feed_id: feed_id.clone(),
                bottom_feed_index,
            })
            .encode(),
        )
        .child(view_slide_content_empty())
}

fn to_media_details_route(media_id: &MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Index {
            media_id: media_id.clone(),
        },
    ))
}

pub fn view_slide(feed_item: &FeedItem) -> Elem {
    view_slide_root()
        .data_feed_index(feed_item.to_feed_index())
        .child(view_slide_content(feed_item))
}

fn view_slide_content(feed_item: &FeedItem) -> Elem {
    match feed_item {
        FeedItem::Media {
            media,
            feed_index: _,
        } => button()
            .class("w-full h-full")
            .data_on_click_push_then_get(&to_media_details_route(&media.id).encode())
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
    }
}
