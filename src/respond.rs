use crate::{
    core::http::response_writer::ResponseWriter, ctx::Ctx, feed, media, req::Req, route::Route,
    user,
};
pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Feed(route) => feed::respond::respond(&ctx, r, route, w).await,

        Route::Account(route) => user::account::respond::respond(&ctx, r, &route, w).await,

        Route::Media(route) => media::respond::respond(&ctx, r, route, w).await,
    }
}
