use super::{feed_::Feed, feed_id::FeedId, feed_item::FeedItem, feed_tags_form, route::Route};
use crate::{
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        session::session_id::SessionId,
        ui::{self, chip::ChipSize, icon, image::Image, top_bar::TopBarRoot},
        unit_of_work::UnitOfWork,
    },
    ctx::Ctx,
    req::Req,
    ui::{bottom_bar::BottomBar, route::Routable},
};

pub const LIMIT: usize = 3;

pub struct ViewModel {
    pub feed_id: FeedId,
    pub feed: Feed,
    pub initial_feed_items: Vec<FeedItem>,
}

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
    feed_id: &FeedId,
) -> Result<(), std::io::Error> {
    w.send_screen(view_screen()).await?;

    respond_screen_contents(ctx, r, w, feed_id).await?;

    Ok(())
}

pub async fn respond_screen_contents(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
    feed_id: &FeedId,
) -> Result<(), std::io::Error> {
    w.send_fragment(view_top_bar_loading_with_link(&feed_id))
        .await?;

    let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

    transact_put_feed(ctx, &r.session_id, &feed).await?;

    let initial_feed_items = get_feed_items(ctx, &feed).await.unwrap_or_default();

    let model = ViewModel {
        feed_id: feed.feed_id.clone(),
        feed,
        initial_feed_items,
    };

    w.send_fragment(view_top_bar(&model)).await?;
    w.send_fragment(view_swiper(&model)).await?;

    Ok(())
}

pub async fn transact_put_feed(
    ctx: &Ctx,
    session_id: &SessionId,
    feed: &Feed,
) -> Result<(), std::io::Error> {
    UnitOfWork::transact(|uow| async move {
        ctx.feed_db.put(uow.clone(), feed.clone()).await?;

        ctx.feed_session_mapping_db
            .put(uow.clone(), session_id.clone(), feed.feed_id.clone())
            .await?;

        Ok(())
    })
    .await?;

    Ok(())
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
    TopBarRoot::default()
        .view()
        .button()
        .class("relative")
        .aria_label("open controls")
        .id("top-bar")
}

fn view_top_bar_loading() -> Elem {
    view_top_bar_root().child(view_open_controls_button())
}

fn view_top_bar_link_root(feed_id: &FeedId) -> Elem {
    view_top_bar_root().data_on(|b| {
        b.click().push_then_sse(
            &feed_tags_form::route::Route::FeedTagsFormScreen {
                feed_id: feed_id.clone(),
            }
            .url(),
        )
    })
}

fn view_top_bar_loading_with_link(feed_id: &FeedId) -> Elem {
    view_top_bar_link_root(&feed_id).child(view_open_controls_button())
}

fn view_top_bar(model: &ViewModel) -> Elem {
    view_top_bar_link_root(&model.feed_id)
        .child(view_tags(&model))
        .child(view_open_controls_button())
}

fn view_slide_content_loading() -> Elem {
    Image::new()
        .view()
        .src(" ")
        .class("w-full h-full object-cover")
}

pub fn view_screen() -> Elem {
    div()
        .id("screen")
        .class("w-full flex-1 flex items-center justify-center flex-col overflow-hidden")
        .data_signal("signal_feed_index", "0")
        .data_signal("signal_true", "true")
        .child(view_top_bar_loading())
        .child(view_swiper_loading())
        .child(view_bottom_bar())
}

fn view_open_controls_button() -> Elem {
    div()
        .class("absolute top-0 right-0 h-full flex items-center justify-center")
        .child(div().class("w-16 h-full from-transparent to-black bg-gradient-to-r"))
        .child(
            div()
                .class("size-16 bg-black flex items-center justify-center")
                .child(ui::icon::solid::pencil("size-6")),
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
                        .signal_checked("signal_true.value")
                        .disabled(true)
                        .id(&tag.encode().to_lowercase())
                        .size(ChipSize::Small)
                        .view()
                })
                .collect::<Vec<Elem>>(),
        )
}

fn view_bottom_bar() -> Elem {
    BottomBar::default().active_home().view().id("bottom-bar")
}

fn view_swiper_root() -> Elem {
    div().class("w-full flex-1 overflow-hidden").id("swiper")
}

fn view_swiper_loading() -> Elem {
    view_swiper_root().child(view_slide_content_loading())
}

fn view_swiper(model: &ViewModel) -> Elem {
    view_swiper_root().child(view_swiper_container(&model))
}

fn view_swiper_container(model: &ViewModel) -> Elem {
    if model.initial_feed_items.len() == 0 {
        return view_swiper_empty();
    }
    ui::swiper::container()
        .swiper_direction_vertical()
        .swiper_slides_per_view("1")
        .class("h-full flex flex-col w-full items-center justify-center overflow-hidden")
        .data_on(|b| b
                .e("swiperslidechange")
                .js("signal_feed_index.value = parseInt(evt?.detail?.[0]?.slides?.[event?.detail?.[0]?.activeIndex]?.getAttribute?.('data-feed-index'), 10)")
                .sse(&Route::ChangedSlide { feed_id: model.feed_id.clone() }.url()))
        .child(view_slides(&model.feed_id, &model.initial_feed_items))
}

fn view_swiper_empty() -> Elem {
    div()
        .class("w-full h-full flex items-center justify-center flex-col gap-4")
        .child(icon::solid::magnifying_glass("size-16"))
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
        .data_intersects(|b| {
            b.sse(
                &Route::IntersectedBottom {
                    feed_id: feed_id.clone(),
                    bottom_feed_index,
                }
                .url(),
            )
        })
        .child(view_slide_content_loading())
}

pub fn view_slide(feed_item: &FeedItem) -> Elem {
    view_slide_root()
        .data_feed_index(feed_item.to_feed_index())
        .child(feed_item.view_slide_content())
}
