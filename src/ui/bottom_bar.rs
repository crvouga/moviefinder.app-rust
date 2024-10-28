use crate::account;
use crate::core::html::*;
use crate::core::ui;
use crate::core::ui::bottom_bar_buttons::BottomButton;
use crate::core::ui::bottom_bar_buttons::BottomButtons;
use crate::feed;
use crate::route;

use super::root::ROOT_SELECTOR;

#[derive(PartialEq, Eq)]
pub enum Active {
    Home,
    Account,
}

pub fn view(active: Active) -> Elem {
    div().class("w-full").child(
        BottomButtons::default()
            .add(
                BottomButton::default()
                    .text("Home")
                    .hx_get(&route::Route::Feed(feed::route::Route::Index).encode())
                    .hx_target(ROOT_SELECTOR)
                    .icon(ui::icon::home("size-8"))
                    .active(active == Active::Home),
            )
            .add(
                BottomButton::default()
                    .text("Account")
                    .hx_get(&route::Route::Account(account::route::Route::Index).encode())
                    .hx_target(ROOT_SELECTOR)
                    .icon(ui::icon::user_circle("size-8"))
                    .active(active == Active::Account),
            )
            .view(),
    )
}
