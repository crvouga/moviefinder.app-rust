use super::{feed_screen, feed_tags_form, route::Route};
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::CoreError> {
    match route {
        Route::FeedScreen(child) => feed_screen::respond::respond(&ctx, r, child, w).await,
        Route::Tags(child) => feed_tags_form::respond::respond(&ctx, r, child, w).await,
    }
}
