use crate::{
    core::{
        html::{div, Html},
        ui::{
            button_group::{ButtonGroup, ButtonGroupMember},
            icon,
        },
    },
    feed::feed_screen,
    user::account_screen,
};

use super::route::AppRoute;

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

    pub fn view(self) -> Html {
        div().class("w-full").child(
            ButtonGroup::default()
                .view()
                .child(
                    ButtonGroupMember::default()
                        .text("Home")
                        .icon(icon::solid::home("size-6"))
                        .active(self.active == Active::Home)
                        .view()
                        .preload_screen(&feed_screen::route::Route::FeedScreenDefault.url())
                        .data_on(|b| {
                            b.press_down()
                                .push_url(&feed_screen::route::Route::FeedScreenDefault.url())
                        }),
                )
                .child(
                    ButtonGroupMember::default()
                        .text("Account")
                        .icon(icon::solid::user_circle("size-6"))
                        .active(self.active == Active::Account)
                        .view()
                        .preload_screen(&account_screen::route::Route::Screen.url())
                        .data_on(|b| {
                            b.press_down()
                                .push_url(&account_screen::route::Route::Screen.url())
                        }),
                ),
        )
    }
}
