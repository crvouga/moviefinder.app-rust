use crate::{
    core::http::response_writer::ResponseWriter, ctx::Ctx, feed, media, req::Req, route::Route,
    user,
};
pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::CoreError> {
    match route {
        Route::Feed(route) => feed::respond::respond(&ctx, r, route, w).await,

        Route::User(route) => user::respond::respond(&ctx, r, &route, w).await,

        Route::Media(route) => media::respond::respond(&ctx, r, route, w).await,
    }
}
