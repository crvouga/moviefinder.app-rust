use super::{
    super::{feed_::Feed, feed_id::FeedId, feed_item::FeedItem, feed_tags_form},
    route::{self, Route},
};
use crate::{
    core::{
        css,
        datastar::datastar::fragments,
        dynamic_data::DynamicData,
        html::*,
        http::response_writer::ResponseWriter,
        js::Js,
        session::session_id::SessionId,
        ui::{self, chip::ChipSize, icon, image::Image, top_bar::TopBarRoot},
        unit_of_work::UnitOfWork,
    },
    ctx::Ctx,
    debug,
    media::{
        self,
        interaction::interaction_form::{
            self, interaction_form_view_config::InteractionFormViewConfig,
        },
        media_id::MediaId,
    },
    req::Req,
    ui::{bottom_bar::BottomBar, route::AppRoute},
};
use std::cmp::max;

const LIMIT: usize = 3;
struct ViewModel {
    feed_id: FeedId,
    feed: Feed,
}

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::FeedScreenDefault => {
            w.send_screen(r, view_screen()).await?;

            let maybe_feed_id = ctx
                .feed_session_mapping_db
                .get(r.session_id.clone())
                .await
                .unwrap_or(None);

            let feed_id = maybe_feed_id.unwrap_or_default();

            w.send_screen(r, view_screen()).await?;

            respond_screen_contents(ctx, r, w, &feed_id).await?;

            Ok(())
        }

        Route::ChangedSlide { feed_id } => {
            let signal_feed_index = r
                .payload
                .get_first("signal_feed_index")
                .and_then(|s| s.parse::<usize>().ok());

            let signal_feed_index = match signal_feed_index {
                None => return Ok(()),

                Some(signal_feed_index) => signal_feed_index,
            };

            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let feed_new = Feed {
                start_index: signal_feed_index,
                ..feed
            };

            put_feed(ctx, &r.session_id, &feed_new).await?;

            Ok(())
        }

        Route::IntersectedBottom { feed_id, .. } => {
            let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

            let signal_feed_index = r
                .payload
                .get_first("signal_feed_index")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_default();

            let feed_with_new_index: Feed = Feed {
                start_index: signal_feed_index + 1,
                ..feed
            };

            let feed_items = get_feed_items(ctx, &feed_with_new_index)
                .await
                .unwrap_or_default();

            if feed_items.is_empty() {
                fragments(view_slide_bottom_empty()).send(w).await?;

                return Ok(());
            }

            for feed_item in &feed_items {
                fragments(view_slide(&feed_item))
                    .selector(&css::selector::id(BOTTOM_ID))
                    .merge_mode_before()
                    .send(w)
                    .await?;
            }

            let user_id = r.user_id(ctx).await?;

            let media_ids = feed_items
                .iter()
                .filter_map(|feed_item| feed_item.to_media_id())
                .collect::<Vec<_>>();

            interaction_form::respond::respond_interaction_form(
                ctx,
                w,
                user_id.clone(),
                media_ids
                    .iter()
                    .map(|media_id| to_interaction_form_view_config(media_id.clone()))
                    .collect(),
            )
            .await?;

            Ok(())
        }
    }
}

fn to_interaction_form_view_config(media_id: MediaId) -> InteractionFormViewConfig {
    InteractionFormViewConfig::default()
        .media_id(media_id)
        .namespace("feed".to_owned())
        .orientation_vertical()
}

pub async fn redirect_to(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
    feed_id: &FeedId,
) -> Result<(), crate::core::error::Error> {
    let r = &Req {
        url: route::Route::FeedScreenDefault.url(),
        ..r.clone()
    };

    w.send_screen(r, view_screen()).await?;

    respond_screen_contents(ctx, r, w, feed_id).await?;

    w.send_script(&Js::push_url(&r.url)).await?;

    Ok(())
}

async fn respond_screen_contents(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
    feed_id: &FeedId,
) -> Result<(), crate::core::error::Error> {
    w.send_fragment(view_top_bar_loading_with_link(&feed_id))
        .await?;

    let feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

    debug!(ctx.log, "respond_screen_contents feed: {:?}", feed);

    put_feed(ctx, &r.session_id, &feed).await?;

    let model = ViewModel {
        feed_id: feed.feed_id.clone(),
        feed,
    };

    w.send_fragment(view_top_bar(&model)).await?;

    w.send_fragment(view_swiper(&model)).await?;

    Ok(())
}

async fn put_feed(
    ctx: &Ctx,
    session_id: &SessionId,
    feed: &Feed,
) -> Result<(), crate::core::error::Error> {
    debug!(ctx.log, "put_feed: {:?}", feed);

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

async fn get_feed_items(
    ctx: &Ctx,
    feed: &Feed,
) -> Result<Vec<FeedItem>, crate::core::error::Error> {
    let query = feed.to_media_query(LIMIT);

    let paginated = ctx.media_db.query(query).await?;

    let feed_items = paginated
        .items
        .into_iter()
        .enumerate()
        .map(|(index, media)| FeedItem::from((media, index + feed.start_index)))
        .collect::<Vec<FeedItem>>();

    Ok(feed_items)
}

fn view_top_bar_root() -> Html {
    TopBarRoot::default()
        .view()
        .button()
        .class("relative")
        .aria_label("open controls")
        .id("top-bar")
}

fn view_top_bar_loading() -> Html {
    view_top_bar_root().child(view_open_controls_button())
}

fn view_top_bar_link_root(feed_id: &FeedId) -> Html {
    view_top_bar_root().data_on(|b| {
        b.press_up().push_url(
            &feed_tags_form::route::Route::FeedTagsFormScreen {
                feed_id: feed_id.clone(),
            }
            .url(),
        )
    })
}

fn view_top_bar_loading_with_link(feed_id: &FeedId) -> Html {
    view_top_bar_link_root(&feed_id).child(view_open_controls_button())
}

fn view_top_bar(model: &ViewModel) -> Html {
    view_top_bar_link_root(&model.feed_id)
        .child(view_tags(&model))
        .child(view_open_controls_button())
}

fn view_screen() -> Html {
    div()
        .id("screen")
        .class("w-full flex-1 flex items-center justify-center flex-col overflow-hidden")
        .child(view_top_bar_loading())
        .child(view_swiper_loading())
        .child(view_bottom_bar())
}

fn view_open_controls_button() -> Html {
    div()
        .class("absolute top-0 right-0 h-full flex items-center justify-center")
        .child(div().class("w-16 h-full from-transparent to-black bg-gradient-to-r"))
        .child(
            div()
                .class("size-16 bg-black flex items-center justify-center")
                .child(ui::icon::solid::pencil("size-6")),
        )
}

fn view_tags(model: &ViewModel) -> Html {
    div()
        .id("tags")
        .class("flex flex-row gap-2 p-4 flex-1 overflow-hidden")
        .data_signal("signal_true", "true")
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
                .collect::<Vec<Html>>(),
        )
}

fn view_bottom_bar() -> Html {
    BottomBar::default().active_home().view().id("bottom-bar")
}

fn view_swiper_root() -> Html {
    div().class("w-full flex-1 overflow-hidden").id("swiper")
}

fn view_swiper_loading() -> Html {
    view_swiper_root().child(view_slide_content_loading())
}

fn view_swiper(model: &ViewModel) -> Html {
    view_swiper_root().child(view_swiper_container(&model))
}

fn view_swiper_container(model: &ViewModel) -> Html {
    ui::swiper::container()
        .class("h-full w-full")
        .data_signal("signal_feed_index", &(max( (model.feed.start_index as i64) - 1, 0).to_string()))
        .data_on(|b| b
                .e("swiperslidechange")
                .js("signal_feed_index.value = parseInt(evt?.detail?.[0]?.slides?.[event?.detail?.[0]?.activeIndex]?.getAttribute?.('data-feed-index'), 10)")
                .sse(&Route::ChangedSlide { feed_id: model.feed_id.clone() }.url()))
        .child(view_slide_bottom(&model.feed_id))
}

fn view_slide_root() -> Html {
    ui::swiper::slide()
}

const BOTTOM_ID: &str = "feed-bottom";

fn view_slide_bottom(feed_id: &FeedId) -> Html {
    view_slide_root()
        .id(BOTTOM_ID)
        .data_intersects(|b| {
            b.sse(
                &Route::IntersectedBottom {
                    feed_id: feed_id.clone(),
                }
                .url(),
            )
        })
        .child(view_slide_content_loading())
}

fn view_slide_bottom_empty() -> Html {
    view_slide_root()
        .id(BOTTOM_ID)
        .class("w-full h-full flex items-center justify-center flex-col gap-4")
        .child(icon::solid::magnifying_glass("size-16"))
        .child(
            div()
                .class("text-2xl font-bold w-full text-center")
                .child_text("No results found"),
        )
}

fn view_slide(feed_item: &FeedItem) -> Html {
    view_slide_root()
        .attr("data-feed-index", &feed_item.to_feed_index().to_string())
        .child(view_slide_content(feed_item))
}

fn view_slide_content_loading() -> Html {
    div().class("w-full h-full flex flex-col").child(
        div().class("w-full flex-1").child(
            Image::new()
                .view()
                .src(" ")
                .class("w-full h-full object-cover"),
        ),
    )
}

fn view_slide_content(feed_item: &FeedItem) -> Html {
    match feed_item {
        FeedItem::Media {
            media,
            feed_index: _,
        } => div()
            .class("w-full h-full flex flex-col content-box relative")
            .child(
                Image::new()
                    .view()
                    .class(
                        "w-full flex-1 overflow-hidden object-cover pointer-cursor content-box border-none outline-none",
                    )
                    .tab_index(0)
                    .role_button()
                    .data_on(|b| {
                        b.click().push_url(
                            &media::details::route::Route::MediaDetailsScreen {
                                media_id: media.id.clone(),
                                back_url: route::Route::FeedScreenDefault.url(),
                            }
                            .url(),
                        )
                    })
                    .aria_label("open media details")
                    .src(media.poster.to_highest_res())
                    .alt(media.title.as_str()),
            )
            .child(
                div().class("absolute bottom-0 right-0").child(
                    to_interaction_form_view_config(media.id.clone()).view()
                )
            ),
    }
}
