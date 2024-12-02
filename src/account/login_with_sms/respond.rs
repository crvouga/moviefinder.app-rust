use children::text;

use super::route::Route;
use crate::{
    account,
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        params::Params,
        ui::{button::Button, text_field::TextField, top_bar::TopBar},
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
            let phone_number = r
                .params
                .get_first("phoneNumber")
                .map(|s| s.clone())
                .unwrap_or_default();

            if phone_number.is_empty() {
                w.send_fragment(view_phone_number_text_field("Phone number is required"))
                    .await?;
                return Ok(());
            }

            let model = ViewModel::Code {
                phone_number: phone_number.clone(),
            };

            w.send_screen_frag(model.view_screen()).await?;

            let new_route = Route::ScreenCode { phone_number };

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
                .trim();

            if code.is_empty() {
                w.send_fragment(view_code_text_field("Code is required"))
                    .await?;
                return Ok(());
            }

            ctx.verify_sms.verify_code(&phone_number, code).await?;

            Ok(())
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
            ViewModel::Phone => self.view_screen_phone(),
            ViewModel::Code { phone_number } => self.view_screen_code(phone_number),
        }
    }

    pub fn view_screen_phone(&self) -> Elem {
        form()
            .class("w-full h-full flex flex-col")
            .data_store("{ phoneNumber: '' }")
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
                    .child(view_phone_number_text_field(""))
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

    pub fn view_screen_code(&self, phone_number: &str) -> Elem {
        form()
            .class("w-full h-full flex flex-col")
            .data_store("{ code: '' }")
            .data_on(|b| {
                b.submit()
                    .prevent_default()
                    .post(&Route::ClickedSendCode.url())
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
                    .child(view_code_text_field(""))
                    .child(
                        div().class("pt-3 w-full").child(
                            Button::default()
                                .label("Send code")
                                .color_primary()
                                .view()
                                .class("w-full")
                                .data_on(|b| b.click().post(&Route::ClickedVerifyCode.url())),
                        ),
                    ),
            )
    }
}

fn view_code_text_field(error: &str) -> Elem {
    TextField::default()
        .label("Code")
        .placeholder("Enter code")
        .input(|e| e.data_model("code").type_("tel"))
        .error(error)
        .view()
        .id("code")
}

fn view_phone_number_text_field(error: &str) -> Elem {
    TextField::default()
        .label("Phone number")
        .placeholder("Enter phone number")
        .input(|e| e.data_model("phoneNumber").type_("tel"))
        .error(error)
        .view()
        .id("phone_number")
}