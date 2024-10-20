use crate::account::route::Route;
use crate::app;
use crate::html::*;
use crate::res::Res;

pub fn respond(route: Route) -> Res {
    match route {
        Route::Index => Res::Html(view_account().render()),
    }
}

pub fn view_account() -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col",
        )],
        &[
            div(&[class("flex-1")], &[text("Account")]),
            app::bottom_nav::view(app::bottom_nav::Active::Account),
        ],
    )
}
