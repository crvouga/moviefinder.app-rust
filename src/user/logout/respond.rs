use super::route::Route;
use crate::{
    core::{
        html::{form, Html},
        http::response_writer::ResponseWriter,
        ui::{
            button::Button,
            drawer::{Drawer, DrawerTitle},
        },
        unit_of_work::uow,
    },
    ctx::Ctx,
    req::Req,
    ui::{bottom_bar_form_buttons::BottomBarFormButtons, route::AppRoute},
    user::account_screen,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::Error> {
    match route {
        Route::ClickedLogout => {
            ctx.user_session_db.zap(uow(), &r.session_id).await?;

            account_screen::redirect_to(ctx, r, w, &None).await?;

            w.send_signal("signal_is_drawer_open", "false").await?;

            w.send_toast_dark("Logged out").await?;

            Ok(())
        }

        Route::LogoutDrawer => {
            w.send_fragment(view_logout_drawer()).await?;

            Ok(())
        }
    }
}

pub fn view_open_logout_drawer_button() -> Button {
    Button::default()
        .color_primary()
        .label("Logout")
        .indicator("signal_indicator_logout")
        .map_button(|e| e.data_on(|b| b.press_down().sse(&Route::LogoutDrawer.url())))
}

fn view_logout_drawer() -> Html {
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
                        .sse(&Route::ClickedLogout.url())
                })
                .data_indicator("signal_is_submitting")
                .id("logout-form")
                .class("w-full h-full flex flex-col items-center")
                .child(
                    DrawerTitle::default()
                        .title("Logout of moviefinder.app?")
                        .view(),
                )
                .child(
                    BottomBarFormButtons::default()
                        .on_cancel(|b| b.press_down().js("signal_is_drawer_open.value = false"))
                        .border(false)
                        .submit_indicator("signal_is_submitting")
                        .submit_label("Logout")
                        .view()
                        .id("logout-form-bottom-bar"),
                ),
        )
        .view()
}
