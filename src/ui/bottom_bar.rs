use crate::account;
use crate::core::html::*;
use crate::core::ui;
use crate::core::ui::bottom_bar_buttons::BottomButton;
use crate::core::ui::bottom_bar_buttons::BottomButtons;
use crate::feed;
use crate::route;

#[derive(PartialEq, Eq)]
pub enum Active {
    Home,
    Account,
}

pub fn view(active: Active) -> Elem {
    div().class("w-full").child(
        BottomButtons::default()
            .view()
            .child(
                BottomButton::default()
                    .text("Home")
                    .icon(ui::icon::home("size-8"))
                    .active(active == Active::Home)
                    .view()
                    .root_push(route::Route::Feed(feed::route::Route::Index)),
            )
            .child(
                BottomButton::default()
                    .text("Account")
                    .icon(ui::icon::user_circle("size-8"))
                    .active(active == Active::Account)
                    .view()
                    .root_push(route::Route::Account(account::route::Route::Index)),
            ),
    )
}
