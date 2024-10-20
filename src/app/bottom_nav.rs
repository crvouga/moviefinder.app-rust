use crate::account;
use crate::feed;
use crate::html::*;
use crate::route;
use crate::ui;

#[derive(PartialEq, Eq)]
pub enum Active {
    Home,
    Account,
}

pub fn view(active: Active) -> Elem {
    div(
        &[class("w-full")],
        &[ui::bottom_bar_buttons::view(&[
            ui::bottom_bar_buttons::Button {
                text: "Home".to_string(),
                href: route::Route::Feed(feed::route::Route::Index).encode(),
                icon: ui::icon::home(&[class("size-8")]),
                active: active == Active::Home,
            },
            ui::bottom_bar_buttons::Button {
                text: "Account".to_string(),
                href: route::Route::Account(account::route::Route::Index).encode(),
                icon: ui::icon::user_circle(&[class("size-8")]),
                active: active == Active::Account,
            },
        ])],
    )
}
