use crate::account;
use crate::core::res::Res;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::req::Req;
use crate::route::Route;

pub async fn respond(ctx: &ctx::Ctx, req: &Req, route: &Route) -> Res {
    match route {
        Route::Feed(route) => feed::respond::respond(&ctx.feed, req, route).await,

        Route::Account(route) => account::respond::respond(route),

        Route::Media(route) => media::respond::respond(ctx, route).await,

        Route::Favicon => Res::empty(),

        Route::RobotsTxt => Res::text("User-agent: *\nDisallow:"),

        Route::OutputCss => Res::css(include_str!("./output.css")),

        Route::Unknown(_route) => {
            Res::redirect_window(Route::Feed(feed::route::Route::DefaultLoad).encode())
        }
    }
}
