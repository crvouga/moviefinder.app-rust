use super::login_with_sms;
use crate::{
    core::{
        html::{form, Elem},
        http::response_writer::ResponseWriter,
        ui::drawer::{Drawer, DrawerTitle},
    },
    ui::{bottom_bar_form::BottomBarForm, route::Routable},
};

impl ResponseWriter {
    pub async fn send_must_login_first(&mut self) -> Result<(), std::io::Error> {
        self.send_fragment(view_must_login_drawer()).await?;
        Ok(())
    }
}

fn view_must_login_drawer() -> Elem {
    Drawer::default()
        .model_open("signal_is_drawer_open")
        .initial_open(true)
        .on_close("signal_is_drawer_open.value = false")
        .content(
            form()
                .data_on(|e| {
                    e.submit()
                        .prevent_default()
                        .js("signal_is_drawer_open.value = false")
                        .push_then_sse(&login_with_sms::route::Route::ScreenPhone.url())
                })
                .id("login-form")
                .class("w-full h-full flex flex-col items-center")
                .child(
                    DrawerTitle::default()
                        .title("You must login first...")
                        .view(),
                )
                .child(
                    BottomBarForm::default()
                        .on_cancel(|b| b.click().js("signal_is_drawer_open.value = false"))
                        .submit_label("Login")
                        .view(),
                ),
        )
        .view()
}
