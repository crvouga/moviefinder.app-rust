use crate::res::Res;
use crate::route;
use crate::route::Route;


pub fn respond(route: Route) -> Res {
    match route {
        Route::Feed(_) => Res::Html("<h1>Feed</h1>".to_string()),

        Route::Account => Res::Html("<h1>Account</h1>".to_string()),

        Route::Unknown => Res::Redirect(route::encode(Route::Feed(feed::))),
    }
}
