use crate::account;
use crate::feed;
use crate::res::Res;
use crate::route;
use crate::route::Route;

pub fn respond(route: Route) -> Res {
    match route {
        Route::Feed(child) => feed::respond::respond(child),

        Route::Account(child) => account::respond::respond(child),

        Route::Unknown => {
            Res::Redirect(route::encode(Route::Account(account::route::Route::Index)))
        }
    }
}
