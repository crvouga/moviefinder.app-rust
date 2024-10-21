use crate::account;
use crate::ctx;
use crate::feed;
use crate::res::Res;
use crate::route;
use crate::route::Route;

pub async fn respond(route: Route, ctx: &ctx::Ctx) -> Res {
    match route {
        Route::Feed(child) => feed::respond::respond(child, ctx).await,

        Route::Account(child) => account::respond::respond(child),

        Route::Unknown => {
            Res::Redirect(route::encode(Route::Account(account::route::Route::Index)))
        }
    }
}
