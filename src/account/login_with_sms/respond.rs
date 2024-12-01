use children::text;

use super::route::Route;
use crate::{
    account,
    core::{
        html::*,
        http::response_writer::ResponseWriter,
        params::Params,
        ui::{button::Button, text_field::TextField},
    },
    ctx::Ctx,
    req::Req,
    route,
    ui::top_bar::TopBar,
};

pub async fn respond(
    _ctx: &Ctx,
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
                .to_owned()
                .map(|s| s.clone())
                .unwrap_or_default();

            if phone_number.is_empty() {
                w.send_frag(view_phone_number_text_field("Phone number is required"))
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

            Ok(())
        }

        Route::ClickedVerifyCode => Ok(()),
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
        div()
            .class("w-full h-full flex flex-col")
            .data_store("{ phoneNumber: '' }")
            .child(
                TopBar::default()
                    .title("Login with phone")
                    .back_button(route::Route::Account(account::route::Route::Screen))
                    .view(),
            )
            .child(
                div()
                    .class("flex-1 p-6 gap-6 flex flex-col")
                    .child(
                        TextField::default()
                            .label("Phone number")
                            .placeholder("Enter phone number")
                            .input(|e| e.data_model("phoneNumber"))
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
                                .data_on(|b| b.click().post(&Route::ClickedSendCode.url())),
                        ),
                    ),
            )
    }

    pub fn view_screen_code(&self, phone_number: &str) -> Elem {
        div()
            .class("w-full h-full flex flex-col")
            .data_store("{ code: '' }")
            .child(
                TopBar::default()
                    .title("Login with phone")
                    .back_button(Route::ScreenPhone.route())
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
                            .input(|e| e.data_model("code"))
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
                                .data_on(|b| b.click().post(&Route::ClickedSendCode.url())),
                        ),
                    ),
            )
    }
}

fn view_phone_number_text_field(error: &str) -> Elem {
    TextField::default()
        .label("Phone number")
        .placeholder("Enter phone number")
        .input(|e| e.data_model("phoneNumber"))
        .error(error)
        .view()
        .id("phone_number")
}
