use children::text;

use super::{route::Route, verify_sms::interface::VerifyCodeError};
use crate::{
    account,
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        params::Params,
        ui::{button::Button, text_field::TextField, toast::Toast, top_bar::TopBar},
    },
    ctx::Ctx,
    req::Req,
    route,
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ScreenPhone => {
            let model = ViewModel::Phone;

            w.send_screen_frag(model.view_screen()).await?;

            Ok(())
        }

        Route::ClickedSendCode => {
            let phone_number_input = r
                .params
                .get_first("phoneNumber")
                .map(|s| s.clone())
                .unwrap_or_default();

            if phone_number_input.is_empty() {
                w.send_signals("{ phoneNumberError: 'Phone number is required' }")
                    .await?;
                return Ok(());
            }

            let phone_number = phone_number_input.trim();

            let model = ViewModel::Code {
                phone_number: phone_number_input.clone(),
            };

            w.send_screen_frag(model.view_screen()).await?;

            let new_route = Route::ScreenCode {
                phone_number: phone_number_input,
            };

            w.send_push_url(&new_route.url()).await?;

            Ok(())
        }

        Route::ScreenCode { phone_number } => {
            let model = ViewModel::Code {
                phone_number: phone_number.to_string(),
            };

            w.send_screen_frag(model.view_screen()).await?;

            ctx.verify_sms.send_code(&phone_number).await?;

            Ok(())
        }

        Route::ClickedVerifyCode { phone_number } => {
            let code = r
                .params
                .get_first("code")
                .map(|s| s.clone())
                .unwrap_or_default()
                .trim()
                .to_string();

            if code.is_empty() {
                w.send_signals("{ codeError: 'Code is required' }").await?;
                return Ok(());
            }

            let verifed = ctx.verify_sms.verify_code(&phone_number, &code).await;

            let error: VerifyCodeError = match verifed {
                Ok(_) => return Ok(()),
                Err(err) => err,
            };

            match error {
                VerifyCodeError::WrongCode => {
                    w.send_signals("{ codeError: 'Wrong code' }").await?;
                    Ok(())
                }
                VerifyCodeError::Error(err) => {
                    w.send_fragment(Toast::error(&err.to_string()).view())
                        .await?;

                    Ok(())
                }
            }
        }
    }
}

impl Route {
    fn route(self) -> route::Route {
        route::Route::Account(account::route::Route::LoginWithSms(self))
    }
    fn url(self) -> String {
        self.route().encode()
    }
}

enum ViewModel {
    Phone,
    Code { phone_number: String },
}

impl ViewModel {
    pub fn view_screen(&self) -> Elem {
        match self {
            ViewModel::Phone => self.view_screen_enter_phone(),
            ViewModel::Code { phone_number } => self.view_screen_enter_code(phone_number),
        }
    }

    pub fn view_screen_enter_phone(&self) -> Elem {
        form()
            .class("w-full h-full flex flex-col")
            .data_store("{ phoneNumber: '', phoneNumberError: '' }")
            .debug_store(false)
            .data_on(|b| {
                b.submit()
                    .prevent_default()
                    .post(&Route::ClickedSendCode.url())
            })
            .child(
                TopBar::default()
                    .title("Login with phone")
                    .back_url(route::Route::Account(account::route::Route::Screen).encode())
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
                                .view()
                                .class("w-full")
                                .type_("submit"),
                        ),
                    ),
            )
    }

    pub fn view_screen_enter_code(&self, phone_number: &str) -> Elem {
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
                                .label("Send code")
                                .color_primary()
                                .view()
                                .class("w-full")
                                .type_("submit"),
                        ),
                    ),
            )
    }
}
