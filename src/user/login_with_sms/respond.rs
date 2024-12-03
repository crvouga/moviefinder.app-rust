use children::text;

use super::{
    core::{self, verify_code, SendCodeError, VerifyCodeError},
    route::Route,
};
use crate::{
    core::{
        datastar::datastar::quote,
        html::*,
        http::response_writer::ResponseWriter,
        params::Params,
        ui::{button::Button, text_field::TextField, top_bar::TopBar},
    },
    ctx::Ctx,
    req::Req,
    route,
    user::{self, account_screen},
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ScreenPhone => {
            w.send_signal("isSubmitting", "false").await?;
            w.send_screen(view_screen_enter_phone()).await?;
            Ok(())
        }

        Route::ClickedSendCode => {
            let phone_number_input = r
                .params
                .get_first("phoneNumber")
                .map(|s| s.clone())
                .unwrap_or_default();

            let sent = core::send_code(ctx, &phone_number_input).await;

            match sent {
                Err(SendCodeError::InvalidPhoneNumber(err)) => {
                    w.send_signal("phoneNumberError", &quote(err.as_str()))
                        .await?;

                    w.send_focus("input").await?;

                    Ok(())
                }

                Err(SendCodeError::Error(err)) => {
                    w.send_toast_error(&err.to_string()).await?;

                    Ok(())
                }

                Ok(()) => {
                    w.send_screen(view_screen_enter_code(&phone_number_input))
                        .await?;

                    w.send_toast_dark(&format!("Code sent to {}", phone_number_input))
                        .await?;

                    let new_route = Route::ScreenCode {
                        phone_number: phone_number_input,
                    };

                    w.send_push_url(&new_route.url()).await?;

                    w.send_focus("input").await?;

                    Ok(())
                }
            }
        }

        Route::ScreenCode { phone_number } => {
            w.send_signal("isSubmitting", "false").await?;

            w.send_screen(view_screen_enter_code(&phone_number)).await?;

            ctx.verify_sms.send_code(&phone_number).await?;

            Ok(())
        }

        Route::ClickedVerifyCode { phone_number } => {
            let code_input = r
                .params
                .get_first("code")
                .map(|s| s.clone())
                .unwrap_or_default()
                .trim()
                .to_string();

            let verified = verify_code(ctx, &r.session_id, phone_number, &code_input).await;

            match verified {
                Err(VerifyCodeError::InvalidCode(err)) => {
                    w.send_signal("codeError", &quote(&err)).await?;

                    w.send_focus("input").await?;

                    Ok(())
                }

                Err(VerifyCodeError::Error(err)) => {
                    w.send_toast_error(&err.to_string()).await?;

                    Ok(())
                }

                Ok(()) => {
                    w.send_toast_dark("Logged in").await?;

                    let route_new = route::Route::User(user::route::Route::Screen);

                    w.send_push_url(&route_new.url()).await?;

                    let user_id = ctx
                        .user_session_db
                        .find_by_session_id(&r.session_id)
                        .await?
                        .map(|s| s.user_id);

                    account_screen::respond_screen(ctx, r, w, &user_id).await?;

                    Ok(())
                }
            }
        }
    }
}

impl Route {
    fn route(self) -> route::Route {
        route::Route::User(user::route::Route::LoginWithSms(self))
    }
    fn url(self) -> String {
        self.route().url()
    }
}

fn view_screen_enter_phone() -> Elem {
    form()
        .class("w-full h-full flex flex-col")
        .data_store("{ phoneNumber: '', phoneNumberError: '' }")
        .debug_store(false)
        .data_on(|b| {
            b.submit()
                .prevent_default()
                .post(&Route::ClickedSendCode.url())
        })
        .id("screen-enter-phone")
        .data_indicator("isSubmitting")
        .child(
            TopBar::default()
                .title("Login with phone")
                .back_url(route::Route::User(user::route::Route::Screen).url())
                .view(),
        )
        .child(
            div()
                .class("flex-1 p-6 gap-6 flex flex-col")
                .child(
                    TextField::default()
                        .label("Phone number")
                        .placeholder("Enter phone number")
                        .input(|e| {
                            e.data_model("phoneNumber")
                                .type_("tel")
                                .data_on(|d| d.input().js("$phoneNumberError = ''"))
                        })
                        .model_error("phoneNumberError")
                        .view()
                        .id("phone_number"),
                )
                .child(
                    div().class("pt-3 w-full").child(
                        Button::default()
                            .label("Send code")
                            .color_primary()
                            .indicator("isSubmitting")
                            .view()
                            .id("send-code")
                            .class("w-full")
                            .type_("submit"),
                    ),
                ),
        )
}

fn view_screen_enter_code(phone_number: &str) -> Elem {
    form()
        .class("w-full h-full flex flex-col")
        .data_store("{ code: '', codeError: '' }")
        .debug_store(false)
        .data_on(|b| {
            b.submit().prevent_default().post(
                &Route::ClickedVerifyCode {
                    phone_number: phone_number.to_string(),
                }
                .url(),
            )
        })
        .id("screen-enter-code")
        .data_indicator("isSubmitting")
        .child(
            TopBar::default()
                .title("Login with phone")
                .back_url(Route::ScreenPhone.url())
                .view(),
        )
        .child(
            div()
                .class("flex-1 p-6 gap-6 flex flex-col")
                .child(
                    div()
                        .class("text-xl w-full")
                        .child(text("Enter the code send to "))
                        .child(span().class("font-bold").child(text(phone_number))),
                )
                .child(
                    TextField::default()
                        .label("Code")
                        .placeholder("Enter code")
                        .input(|e| {
                            e.data_model("code")
                                .type_("tel")
                                .data_on(|d| d.input().js("$codeError = ''"))
                        })
                        .model_error("codeError")
                        .view()
                        .id("code"),
                )
                .child(
                    div().class("pt-3 w-full").child(
                        Button::default()
                            .label("Verify code")
                            .color_primary()
                            .indicator("isSubmitting")
                            .view()
                            .id("verify-code")
                            .class("w-full")
                            .type_("submit"),
                    ),
                ),
        )
}
