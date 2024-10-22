use super::feed_item::FeedItem;
use crate::{
    core::{
        html::*,
        hx,
        query::{Filter, Query},
        res::Res,
        ui,
    },
    ctx,
    feed::route::Route,
    media::{self, media_id::MediaId},
    route,
    ui::{bottom_nav, root::ROOT_SELECTOR},
};

pub async fn respond(route: &Route, ctx: &ctx::Ctx) -> Res {
    match route {
        Route::Index => Res::Html(view_feed()),

        Route::LoadMore => {
            let query = Query {
                limit: 10,
                offset: 0,
                filter: Filter::None,
            };

            let queried = ctx.media_db.query(query).await;

            match queried {
                Ok(paginated) => {
                    let feed_items = paginated
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(index, media)| FeedItem::from((media, index as i32)))
                        .collect::<Vec<FeedItem>>();

                    Res::Html(view_feed_items(&feed_items))
                }

                Err(err) => Res::Html(ui::error::page(&err)),
            }
        }
    }
}

fn view_feed() -> Elem {
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
                ],
                &[view_load_initial()],
            ),
            bottom_nav::view(bottom_nav::Active::Home),
        ],
    )
}

fn view_feed_items(feed_items: &Vec<FeedItem>) -> Elem {
    fragment(
        &feed_items
            .into_iter()
            .map(view_feed_item)
            .collect::<Vec<Elem>>(),
    )
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
                attr(&"data-feed-index", &feed_index.to_string()),
            ],
            &[button(
                &[
                    class("w-full h-full"),
                    hx::get(&to_media_details_route(&media.media_id).encode()),
                    hx::Trigger::Click.attr(),
                    hx::Preload::MouseDown.attr(),
                    hx::Swap::InnerHtml.attr(),
                    hx::push_url("true"),
                    hx::target(&ROOT_SELECTOR),
                ],
                &[ui::image::view(&[
                    class("w-full h-full object-cover"),
                    width(&"100%"),
                    height(&"100%"),
                    src(&media.media_poster.to_highest_res()),
                ])],
            )],
        ),
    }
}

fn view_load_initial() -> Elem {
    div(
        &[
            class("flex-1 flex flex-col items-center justify-center"),
            hx::get(&route::Route::Feed(Route::LoadMore).encode()),
            hx::Trigger::Load.attr(),
            hx::Swap::OuterHtml.attr(),
        ],
        &[ui::icon::spinner(&[class("size-16 animate-spin")])],
    )
}
