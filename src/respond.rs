use crate::account;
use crate::core::res::Res;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::route::Route;

pub async fn respond(route: &Route, ctx: &ctx::Ctx) -> Res {
    match route {
        Route::Feed(child) => feed::respond::respond(child, ctx).await,

        Route::Account(child) => account::respond::respond(child),

        Route::Media(child) => media::respond::respond(child, ctx).await,

        Route::Favicon => Res::Empty,

        Route::RobotsTxt => Res::Text("User-agent: *\nDisallow:".to_owned()),

        Route::Unknown(_route) => Res::Redirect(Route::Feed(feed::route::Route::Index).encode()),
    }
}
