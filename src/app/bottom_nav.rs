use crate::account;
use crate::feed;
use crate::html::*;
use crate::route;
use crate::ui;

pub fn view() -> Elem {
    div(
        &[class("w-full")],
        &[ui::bottom_bar_buttons::view(&[
            ui::bottom_bar_buttons::Button {
                text: "Feed".to_string(),
                href: route::Route::Feed(feed::route::Route::Index).encode(),
                icon: ui::icon::spinner(&[class("size-8")]),
                active: true,
            },
            ui::bottom_bar_buttons::Button {
                text: "Account".to_string(),
                href: route::Route::Account(account::route::Route::Index).encode(),
                icon: ui::icon::spinner(&[class("size-8")]),
                active: true,
            },
        ])],
    )
}
