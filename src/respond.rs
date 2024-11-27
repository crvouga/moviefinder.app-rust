use crate::account;
use crate::core::http::response_writer::ResponseWriter;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::req::Req;
use crate::route::Route;

pub async fn respond(
    ctx: &ctx::Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Feed(route) => feed::respond::respond(&ctx.feed, r, route, w).await,

        Route::Account(route) => account::respond::respond(&ctx, r, &route, w).await,

        Route::Media(route) => media::respond::respond(&ctx, r, route, w).await,

        Route::Favicon => Ok(()),

        Route::RobotsTxt => w.text("User-agent: *\nDisallow:").await,

        Route::OutputCss => w.css(include_bytes!("./output.css")).await,

        Route::Unknown(_route) => {
            feed::respond::respond(&ctx.feed, r, &feed::route::Route::Default, w).await
        }
    }
}
