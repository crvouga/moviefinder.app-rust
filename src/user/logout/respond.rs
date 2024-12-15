use super::route::Route;
use crate::{
    core::{
        html::{form, Elem},
        http::response_writer::ResponseWriter,
        ui::drawer::{Drawer, DrawerTitle},
        unit_of_work::uow,
    },
    ctx::Ctx,
    req::Req,
    ui::{bottom_bar_form::BottomBarForm, route::Routable},
    user::account_screen,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
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

fn view_logout_drawer() -> Elem {
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
                    BottomBarForm::default()
                        .on_cancel(|b| b.click().js("signal_is_drawer_open.value = false"))
                        .submit_indicator("signal_is_submitting")
                        .submit_label("Logout")
                        .view()
                        .id("logout-form-bottom-bar"),
                ),
        )
        .view()
}
