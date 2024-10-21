use super::feed_item::FeedItem;
use crate::app;
use crate::core::pagination::Paginated;
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

            let media = queried.unwrap_or(Paginated::empty()).items;

            let feed_items = media
                .into_iter()
                .map(|media| FeedItem::from((media, 0)))
                .collect::<Vec<FeedItem>>();

            Res::Html(view_feed_items(&feed_items).render())
        }
    }
}

fn view_feed() -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col",
        )],
        &[
            div(
                &[class("flex-1 flex flex-col items-center justify-center")],
                &[view_load_initial()],
            ),
            app::bottom_nav::view(app::bottom_nav::Active::Home),
        ],
    )
}

fn view_feed_items(feed_items: &Vec<FeedItem>) -> Elem {
    div(
        &[class("flex-1 flex flex-col items-center justify-center")],
        &feed_items
            .iter()
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
