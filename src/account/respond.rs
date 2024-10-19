use crate::account::route::Route;
use crate::res::Res;

pub fn respond(route: Route) -> Res {
    match route {
        Route::Index => Res::Html("<h1>Hello Account</h1>".to_string()),
    }
}
