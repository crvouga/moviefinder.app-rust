use crate::account;
use crate::core::res::Res;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::req::Req;
use crate::route::Route;

pub async fn respond(ctx: &ctx::Ctx, req: &Req, route: &Route) -> Res {
    match route {
        Route::Feed(route) => feed::respond::respond(ctx, req, route).await,

        Route::Account(route) => account::respond::respond(route),

        Route::Media(route) => media::respond::respond(ctx, route).await,

        Route::Favicon => Res::Empty,

        Route::RobotsTxt => Res::Text("User-agent: *\nDisallow:".to_owned()),

        Route::Unknown(_route) => Res::Redirect(Route::Feed(feed::route::Route::Index).encode()),
    }
}
