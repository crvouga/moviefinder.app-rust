use super::{super::login_with_sms, route::Route};
use crate::{
    core::{
        html::{form, frag, Html},
        http::response_writer::ResponseWriter,
        ui::drawer::{Drawer, DrawerBody, DrawerTitle},
    },
    ctx::Ctx,
    req::Req,
    ui::{bottom_bar_form_buttons::BottomBarFormButtons, route::AppRoute},
};

impl ResponseWriter {
    pub async fn respond_login_drawer(
        &mut self,
        message: &str,
    ) -> Result<(), crate::core::error::Error> {
        self.send_fragment(view_must_login_drawer(message)).await?;

        Ok(())
    }
}

pub async fn respond(
    _ctx: &Ctx,
    _r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::LoginDrawer => {
            w.send_fragment(view_must_login_drawer("")).await?;

            Ok(())
        }
    }
}

fn view_must_login_drawer(message: &str) -> Html {
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
                        .push_url(&login_with_sms::route::Route::ScreenPhone.url())
                })
                .id("login-form")
                .class("w-full h-full flex flex-col items-center")
                .child(DrawerTitle::default().title("Login required").view())
                .child(if message.is_empty() {
                    frag()
                } else {
                    DrawerBody::default().content(message).view()
                })
                .child(
                    BottomBarFormButtons::default()
                        .border(false)
                        .on_cancel(|b| b.press_down().js("signal_is_drawer_open.value = false"))
                        .submit_label("Login")
                        .view(),
                ),
        )
        .view()
}
