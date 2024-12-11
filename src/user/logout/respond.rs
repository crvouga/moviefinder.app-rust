use super::route::Route;
use crate::{
    core::{
        html::{children::text, div, Elem},
        http::response_writer::ResponseWriter,
        ui::{button::Button, drawer::Drawer},
        unit_of_work::uow,
    },
    ctx::Ctx,
    req::Req,
    ui::route::Routable,
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

            account_screen::respond(ctx, r, w, &None).await?;

            w.send_signal("signal_is_logout_drawer_open", "false")
                .await?;

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
        .model_open("signal_is_logout_drawer_open")
        .initial_open(true)
        .on_close("signal_is_logout_drawer_open.value = false")
        .content(
            div()
                .class("w-full h-full p-6 gap-6 flex flex-col items-center")
                .child(
                    div()
                        .class("text-3xl font-bold text-left w-full")
                        .child(text("Logout of moviefinder.app?")),
                )
                .child(
                    div()
                        .class("flex gap-3 w-full pb-3")
                        .child(
                            Button::default()
                                .color_gray()
                                .label("Cancel")
                                .view()
                                .data_on(|b| {
                                    b.click().js("signal_is_logout_drawer_open.value = false")
                                })
                                .class("flex-1"),
                        )
                        .child(
                            Button::default()
                                .color_primary()
                                .label("Logout")
                                .indicator("signal_is_logging_out")
                                .view()
                                .class("flex-1")
                                .id("logout-button")
                                .data_on(|b| b.click().sse(&Route::ClickedLogout.url())),
                        ),
                ),
        )
        .view()
}
