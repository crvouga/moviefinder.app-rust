use crate::account::route::Route;
use crate::core::html::*;
use crate::core::res::Res;
use crate::ui::bottom_bar;

pub fn respond(route: &Route) -> Res {
    match route {
        Route::Index => view_account().into(),
    }
}

pub fn view_account() -> Elem {
    div(
        &[class(
            "w-full flex-1 flex items-center justify-center flex-col",
        )],
        &[
            div(&[class("flex-1")], &[text("Account")]),
            bottom_bar::view(bottom_bar::Active::Account),
        ],
    )
}
