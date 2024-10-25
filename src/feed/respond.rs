use super::{core::Feed, feed_id::FeedId, feed_item::FeedItem, route::Route};
use crate::{
    core::{
        html::*,
        hx,
        query::{Filter, Query},
        res::Res,
        ui,
    },
    ctx,
    media::{self, media_id::MediaId},
    req::Req,
    route,
    ui::{bottom_nav, root::ROOT_SELECTOR},
    user_session::session_id::SessionId,
};

pub async fn respond(ctx: &ctx::Ctx, req: &Req, route: &Route) -> Res {
    match route {
        Route::Index => {
            let feed_id = ctx
                .session_feed_mapping_db
                .get(req.session_id.clone())
                .await
                .unwrap_or(None)
                .unwrap_or_default();

            let feed_new = ctx
                .feed_db
                .get(feed_id.clone())
                .await
                .unwrap_or(None)
                .unwrap_or_default();

            put_feed(ctx, req.session_id.clone(), &feed_new).await;

            view_feed(feed_new.feed_id).into()
        }

        Route::ChangedSlide(feed_id) => {
            let slide_index_new = req
                .form_data
                .get("feedIndex")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_default();

            let feed = ctx
                .feed_db
                .get(feed_id.clone())
                .await
                .unwrap_or(None)
                .unwrap_or_default();

            let feed_new = Feed {
                active_index: slide_index_new,
                ..feed
            };

            put_feed(ctx, req.session_id.clone(), &feed_new).await;

            Res::Empty
        }

        Route::LoadMore(feed_id) => {
            let feed = ctx
                .feed_db
                .get(feed_id.clone())
                .await
                .unwrap_or(None)
                .unwrap_or_default();

            let query = Query {
                limit: 10,
                offset: feed.active_index,
                filter: Filter::None,
            };

            let queried = ctx.media_db.query(query).await;

            match queried {
                Err(err) => ui::error::page(&err).into(),

                Ok(paginated) => {
                    let feed_items = paginated
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(index, media)| FeedItem::from((media, index as i32)))
                        .collect::<Vec<FeedItem>>();

                    view_feed_items(&feed_items).into()
                }
            }
        }
    }
}

async fn put_feed(ctx: &ctx::Ctx, session_id: SessionId, feed: &Feed) {
    ctx.feed_db.put(feed.clone()).await.unwrap_or(());
    ctx.session_feed_mapping_db
        .put(session_id.clone(), feed.feed_id.clone())
        .await
        .unwrap_or(());
}

fn view_feed(feed_id: FeedId) -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col overflow-hidden",
        )],
        &[
            ui::swiper::container(
                &[
                    class(
                        "flex-1 flex flex-col w-full items-center justify-center overflow-hidden",
                    ),
                    ui::swiper::Direction::Vertical.into(),
                    ui::swiper::slides_per_view("1"),
                    hx::Trigger::Custom("swiperslidechange from:swiper-container".to_string())
                        .into(),
                    hx::Swap::None.into(),
                    hx::post(
                        route::Route::Feed(Route::ChangedSlide(feed_id.clone()))
                            .encode()
                            .as_str(),
                    ),
                    hx::vals(
                        r#"
                        js:{
                            feedIndex: parseInt(event?.detail?.[0]?.slides?.[event?.detail?.[0]?.activeIndex]?.getAttribute?.('data-feed-index'), 10)
                        }
                        "#,
                    ),
                ],
                &[view_load_initial(feed_id.clone())],
            ),
            bottom_nav::view(bottom_nav::Active::Home),
        ],
    )
}

fn view_feed_items(feed_items: &[FeedItem]) -> Elem {
    fragment(&feed_items.iter().map(view_feed_item).collect::<Vec<Elem>>())
}

fn to_media_details_route(media_id: &MediaId) -> route::Route {
    route::Route::Media(media::route::Route::Details(
        media::details::route::Route::Index {
            media_id: media_id.clone(),
        },
    ))
}

fn view_feed_item(feed_item: &FeedItem) -> Elem {
    match feed_item {
        FeedItem::Media { media, feed_index } => ui::swiper::slide(
            &[
                class("w-full h-full flex flex-col items-center justify-center cursor-pointer"),
                attr("data-feed-index", &feed_index.to_string()),
            ],
            &[button(
                &[
                    class("w-full h-full"),
                    hx::get(&to_media_details_route(&media.media_id).encode()),
                    hx::Trigger::Click.into(),
                    hx::Preload::MouseDown.into(),
                    hx::Swap::InnerHtml.into(),
                    hx::push_url("true"),
                    hx::target(ROOT_SELECTOR),
                ],
                &[ui::image::view(&[
                    class("w-full h-full object-cover"),
                    width("100%"),
                    height("100%"),
                    src(media.media_poster.to_highest_res()),
                ])],
            )],
        ),
    }
}

fn view_load_initial(feed_id: FeedId) -> Elem {
    div(
        &[
            class("flex-1 flex flex-col items-center justify-center"),
            hx::get(&route::Route::Feed(Route::LoadMore(feed_id)).encode()),
            hx::Trigger::Load.into(),
            hx::Swap::OuterHtml.into(),
        ],
        &[ui::icon::spinner(&[class("size-16 animate-spin")])],
    )
}
