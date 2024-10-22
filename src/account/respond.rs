use crate::account::route::Route;
use crate::core::html::*;
use crate::core::res::Res;
use crate::ui::bottom_nav;

pub fn respond(route: &Route) -> Res {
    match route {
        Route::Index => Res::Html(view_account()),
    }
}

pub fn view_account() -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col",
        )],
        &[
            div(&[class("flex-1")], &[text("Account")]),
            bottom_nav::view(bottom_nav::Active::Account),
        ],
    )
}
