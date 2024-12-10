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
        phone_number::{country_code::PhoneNumerCountryCode, ensure_country_code},
        ui::{
            button::Button,
            select::{Select, SelectOption},
            text_field::TextField,
            top_bar::TopBar,
        },
        unstructed_data::UnstructedData,
    },
    ctx::Ctx,
    req::Req,
    route,
    ui::route::Routable,
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

            w.send_screen(view_screen_enter_phone(vec![])).await?;

            let country_codes = ctx.phone_number_country_code_db.get_all().await;

            let initial_country_code = country_codes
                .first()
                .map(|c| c.country_code.clone())
                .unwrap_or_default();

            w.send_fragment(view_select_country_code_input(country_codes))
                .await?;

            w.send_signal("countryCode", &initial_country_code).await?;

            Ok(())
        }

        Route::ClickedSendCode => {
            w.send_signal("phoneNumberError", "''").await?;

            let phone_number_input = r
                .params
                .get_first("phoneNumber")
                .map(|s| s.clone())
                .unwrap_or_default();

            let country_code_input = r
                .params
                .get_first("countryCode")
                .map(|s| s.clone())
                .unwrap_or_default();

            let phone_number_input = ensure_country_code(&country_code_input, &phone_number_input);

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
                    w.send_toast_dark(&format!("Code sent to {}", phone_number_input))
                        .await?;

                    w.send_screen(view_screen_enter_code(&phone_number_input))
                        .await?;

                    w.send_push_url(
                        &Route::ScreenCode {
                            phone_number: phone_number_input,
                        }
                        .url(),
                    )
                    .await?;

                    w.send_focus("input").await?;

                    Ok(())
                }
            }
        }

        Route::ScreenCode { phone_number } => {
            w.send_signal("isSubmitting", "false").await?;

            w.send_screen(view_screen_enter_code(&phone_number)).await?;

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

                    w.send_push_url(&user::route::Route::AccountScreen.url())
                        .await?;

                    let user_id = ctx
                        .user_session_db
                        .find_by_session_id(&r.session_id)
                        .await?
                        .map(|s| s.user_id);

                    account_screen::respond(ctx, r, w, &user_id).await?;

                    Ok(())
                }
            }
        }
    }
}

fn view_screen_enter_phone(country_codes: Vec<PhoneNumerCountryCode>) -> Elem {
    form()
        .class("w-full h-full flex flex-col")
        .data_store(
            "{ phoneNumber: '', phoneNumberError: '', countryCode: '', countryCodeError: '' }",
        )
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
                .back_url(route::Route::User(user::route::Route::AccountScreen).url())
                .view(),
        )
        .child(
            div()
                .class("flex-1 p-6 gap-8 flex flex-col")
                .child(
                    div()
                        .class("gap-4 flex flex-col")
                        .child(view_select_country_code_input(country_codes))
                        .child(
                            TextField::default()
                                .label("Phone number")
                                .placeholder("Enter phone number")
                                .map_input(|e| {
                                    e.data_model("phoneNumber")
                                        .type_("tel")
                                        .autocomplete("tel")
                                        .data_on(|d| d.input().js("$phoneNumberError = ''"))
                                })
                                .model_error("phoneNumberError")
                                .view()
                                .id("phone_number"),
                        ),
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

fn view_select_country_code_input(country_codes: Vec<PhoneNumerCountryCode>) -> Elem {
    let options: Vec<SelectOption> = country_codes
        .iter()
        .map(|c| {
            SelectOption::default().value(&c.country_code).label(
                &format!(
                    "{} {} (+{})",
                    c.to_emoji().unwrap_or("".to_string()),
                    &c.country_name,
                    &c.country_code
                )
                .trim(),
            )
        })
        .collect();

    let options = if options.is_empty() {
        vec![SelectOption::default().value("").label("Loading...")]
    } else {
        options
    };

    Select::default()
        .label("Country Code")
        .placeholder("Select country code")
        .map_select(|e| {
            e.data_model("countryCode")
                .data_on(|d| d.change().js("$countryCodeError = ''"))
        })
        .options(options)
        .model_error("countryCodeError")
        .select_id("country-code-select")
        .view()
        .id("country_code")
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
                .class("flex-1 p-6 gap-8 flex flex-col")
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
                        .map_input(|e| {
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
