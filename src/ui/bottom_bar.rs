use crate::{
    core::{
        html::{div, Elem},
        ui::{
            button_group::{ButtonGroup, ButtonGroupMember},
            icon,
        },
    },
    feed::feed_screen,
    route, user,
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

    pub fn view(self) -> Elem {
        div().class("w-full").child(
            ButtonGroup::default()
                .view()
                .child(
                    ButtonGroupMember::default()
                        .text("Home")
                        .icon(icon::solid::home("size-6"))
                        .active(self.active == Active::Home)
                        .view()
                        .data_on(|b| {
                            b.press_down()
                                .push_then_sse(&feed_screen::route::Route::FeedScreenDefault.url())
                        }),
                )
                .child(
                    ButtonGroupMember::default()
                        .text("Account")
                        .icon(icon::solid::user_circle("size-6"))
                        .active(self.active == Active::Account)
                        .view()
                        .data_on(|b| {
                            b.press_down().push_then_sse(
                                &route::Route::User(user::route::Route::AccountScreen).url(),
                            )
                        }),
                ),
        )
    }
}
