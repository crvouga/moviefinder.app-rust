use crate::app;
use crate::feed::route::Route;
use crate::html::*;
use crate::res::Res;

pub fn respond(route: Route) -> Res {
    match route {
        Route::Index => Res::Html(view_feed().render(0)),
    }
}

pub fn view_feed() -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col",
        )],
        &[div(&[class("flex-1")], &[]), app::bottom_nav::view()],
    )
}
