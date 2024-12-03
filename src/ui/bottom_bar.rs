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

    pub fn active_home(mut self) -> Self {
        self.active = Active::Home;
        self
    }

    pub fn active_account(mut self) -> Self {
        self.active = Active::Account;
        self
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
                                &route::Route::Feed(feed::route::Route::ScreenDefault).url(),
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
                                &route::Route::User(user::route::Route::Screen).url(),
                            )
                        }),
                ),
        )
    }
}
