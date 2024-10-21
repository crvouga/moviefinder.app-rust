use crate::account;
use crate::core::res::Res;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::route::Route;

pub async fn respond(route: Route, ctx: &ctx::Ctx) -> Res {
    match route {
        Route::Feed(child) => feed::respond::respond(child, ctx).await,

        Route::Account(child) => account::respond::respond(child),

        Route::Unknown => Res::Redirect(Route::Account(account::route::Route::Index).encode()),

        Route::Media(child) => media::respond::respond(child, ctx).await,
    }
}
