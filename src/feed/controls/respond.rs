use super::route::Route;
use crate::{
    core::{html::*, res::Res},
    ctx,
    feed::{self, core::Feed, feed_id::FeedId},
    req::Req,
    route,
    ui::top_bar,
};

pub async fn respond(ctx: &ctx::Ctx, req: &Req, feed_id: &FeedId, route: &Route) -> Res {
    match route {
        Route::Index => {
            let feed = ctx.feed_db.get_with_fallback(feed_id.clone()).await;

            view_controls(ctx, req, &feed).into()
        }
    }
}

fn view_controls(_ctx: &ctx::Ctx, _req: &Req, _feed: &Feed) -> Elem {
    div(
        &[class("w-full h-full flex flex-col")],
        &[top_bar::root(
            &[],
            &[
                top_bar::empty(),
                top_bar::title("Controls"),
                top_bar::cancel_button(route::Route::Feed(feed::route::Route::Index)),
            ],
        )],
    )
}
