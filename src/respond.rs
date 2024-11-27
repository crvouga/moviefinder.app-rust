use crate::{
    account,
    core::http::{request::Request, response_writer::ResponseWriter},
    ctx::Ctx,
    feed, media,
    route::Route,
};
pub async fn respond(
    ctx: &Ctx,
    r: &Request,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Feed(route) => feed::respond::respond(&ctx.feed, r, route, w).await,

        Route::Account(route) => account::respond::respond(&ctx, r, &route, w).await,

        Route::Media(route) => media::respond::respond(&ctx, r, route, w).await,
    }
}
