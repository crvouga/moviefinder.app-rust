use super::feed_item::FeedItem;
use crate::app;
use crate::ctx;
use crate::feed::route::Route;
use crate::html::*;
use crate::hx;
use crate::res::Res;
use crate::route;
use crate::ui;

pub fn respond(route: Route, ctx: &ctx::Ctx) -> Res {
    match route {
        Route::Index => Res::Html(view_feed().render()),

        Route::LoadMore => {
            let queried = ctx.media_db.query();

            match queried {
                Ok(paginated) => {
                    let feed_items = paginated
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(index, media)| FeedItem::from((media, index as i32)))
                        .collect::<Vec<FeedItem>>();

                    Res::Html(view_feed_items(&feed_items).render())
                }

                Err(err) => Res::Html(ui::error::page(&err).render()),
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
            app::bottom_nav::view(app::bottom_nav::Active::Home),
        ],
    )
}

fn view_feed_items(feed_items: &Vec<FeedItem>) -> Elem {
    fragment(
        &feed_items
            .into_iter()
            .map(|feed_item| feed_item.into())
            .collect::<Vec<Elem>>(),
    )
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
