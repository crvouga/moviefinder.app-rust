use crate::{
    core::{
        html::{div, Elem},
        ui::{
            bottom_bar_buttons::{BottomButton, BottomButtons},
            icon,
        },
    },
    feed, route, user,
};

use super::route::Routable;

#[derive(PartialEq, Eq, Default)]
pub enum Active {
    #[default]
    Home,
    Account,
}

#[derive(Default)]
pub struct BottomBar {
    active: Active,
}

impl BottomBar {
    pub fn active(mut self, value: Active) -> Self {
        self.active = value;
        self
    }

    pub fn active_home(self) -> Self {
        self.active(Active::Home)
    }

    pub fn active_account(self) -> Self {
        self.active(Active::Account)
    }

    pub fn view(self) -> Elem {
        div().class("w-full").child(
            BottomButtons::default()
                .view()
                .child(
                    BottomButton::default()
                        .text("Home")
                        .icon(icon::home("size-6"))
                        .active(self.active == Active::Home)
                        .view()
                        .data_on(|b| {
                            b.click().push_then_get(
                                &route::Route::Feed(feed::route::Route::FeedScreenDefault).url(),
                            )
                        }),
                )
                .child(
                    BottomButton::default()
                        .text("Account")
                        .icon(icon::user_circle("size-6"))
                        .active(self.active == Active::Account)
                        .view()
                        .data_on(|b| {
                            b.click().push_then_get(
                                &route::Route::User(user::route::Route::AccountScreen).url(),
                            )
                        }),
                ),
        )
    }
}
