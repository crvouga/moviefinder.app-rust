use children::text;

use super::{
    core::{self, verify_code, SendCodeError, VerifyCodeError},
    route::Route,
};
use crate::{
    core::{
        dynamic_data::DynamicData,
        html::*,
        http::response_writer::ResponseWriter,
        js::Js,
        phone_number::{country_code::PhoneNumerCountryCode, ensure_country_code},
        ui::{
            button::Button,
            select::{Select, SelectOption},
            text_field::TextField,
            top_bar::TopBar,
        },
    },
    ctx::Ctx,
    req::Req,
    route,
    ui::route::Routable,
    user::{self, account_screen},
};

const SIGNAL_IS_SUBMITTING: &str = "signal_is_submitting";
const SIGNAL_PHONE_NUMBER: &str = "signal_phone_number";
const SIGNAL_PHONE_NUMBER_ERROR: &str = "signal_phone_number_error";
const SIGNAL_COUNTRY_CODE: &str = "signal_country_code";
const SIGNAL_COUNTRY_CODE_ERROR: &str = "signal_country_code_error";
const SIGNAL_CODE: &str = "signal_code";
const SIGNAL_CODE_ERROR: &str = "signal_code_error";

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::ScreenPhone => {
            w.send_signal(SIGNAL_IS_SUBMITTING, "false").await?;

            w.send_screen(view_screen_enter_phone(vec![])).await?;

            let country_codes = ctx.phone_number_country_code_db.get_all().await;

            let initial_country_code = country_codes
                .first()
                .map(|c| c.country_code.clone())
                .unwrap_or_default();

            w.send_fragment(view_select_country_code_input(country_codes))
                .await?;

            w.send_signal(SIGNAL_COUNTRY_CODE, &initial_country_code)
                .await?;

            Ok(())
        }

        Route::ClickedSendCode => {
            w.send_signal(SIGNAL_PHONE_NUMBER_ERROR, "''").await?;

            let phone_number_input = r
                .payload
                .get_first(SIGNAL_PHONE_NUMBER)
                .map(|s| s.clone())
                .unwrap_or_default();

            if phone_number_input.trim().is_empty() {
                w.send_signal(
                    SIGNAL_PHONE_NUMBER_ERROR,
                    &Js::quote("Phone number is required"),
                )
                .await?;

                w.send_script(&Js::focus("input")).await?;

                return Ok(());
            }

            let country_code_input = r
                .payload
                .get_first(SIGNAL_COUNTRY_CODE)
                .map(|s| s.clone())
                .unwrap_or_default();

            let country_codes = ctx.phone_number_country_code_db.get_all().await;

            let phone_number_input = ensure_country_code(
                country_codes
                    .iter()
                    .map(|c| c.country_code.clone())
                    .collect(),
                &country_code_input,
                &phone_number_input,
            );

            let sent = core::send_code(ctx, &phone_number_input).await;

            match sent {
                Err(SendCodeError::InvalidPhoneNumber(err)) => {
                    w.send_signal(SIGNAL_PHONE_NUMBER_ERROR, &Js::quote(err.as_str()))
                        .await?;

                    w.send_script(&Js::focus("input")).await?;

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

                    w.send_script(&Js::push_url(
                        &Route::ScreenCode {
                            phone_number: phone_number_input,
                        }
                        .url(),
                    ))
                    .await?;

                    w.send_script(&Js::focus("input")).await?;

                    Ok(())
                }
            }
        }

        Route::ScreenCode { phone_number } => {
            w.send_signal(SIGNAL_IS_SUBMITTING, "false").await?;

            w.send_screen(view_screen_enter_code(&phone_number)).await?;

            Ok(())
        }

        Route::ClickedVerifyCode { phone_number } => {
            let code_input = r
                .payload
                .get_first(SIGNAL_CODE)
                .map(|s| s.clone())
                .unwrap_or_default()
                .trim()
                .to_string();

            let verified = verify_code(ctx, &r.session_id, phone_number, &code_input).await;

            match verified {
                Err(VerifyCodeError::InvalidCode(err)) => {
                    w.send_signal(SIGNAL_CODE_ERROR, &Js::quote(&err)).await?;

                    w.send_script(&Js::focus("input")).await?;

                    Ok(())
                }

                Err(VerifyCodeError::Error(err)) => {
                    w.send_toast_error(&err.to_string()).await?;

                    Ok(())
                }

                Ok(()) => {
                    w.send_toast_dark("Logged in").await?;

                    let user_id = ctx
                        .user_session_db
                        .find_by_session_id(&r.session_id)
                        .await?
                        .map(|s| s.user_id);

                    account_screen::redirect_to(ctx, r, w, &user_id).await?;

                    Ok(())
                }
            }
        }
    }
}

fn view_screen_enter_phone(country_codes: Vec<PhoneNumerCountryCode>) -> Elem {
    form()
        .class("w-full h-full flex flex-col")
        .data_signal(SIGNAL_PHONE_NUMBER, "")
        .data_signal(SIGNAL_PHONE_NUMBER_ERROR, "")
        .data_signal(SIGNAL_COUNTRY_CODE, "")
        .data_signal(SIGNAL_COUNTRY_CODE_ERROR, "")
        .child_signals_json(false)
        .data_on(|b| {
            b.submit()
                .prevent_default()
                .sse(&Route::ClickedSendCode.url())
        })
        .id("screen-enter-phone")
        .data_indicator(SIGNAL_IS_SUBMITTING)
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
                                    e.data_bind(SIGNAL_PHONE_NUMBER)
                                        .type_("tel")
                                        .autocomplete("tel")
                                        .data_on(|d| {
                                            d.input().js(&Js::assign(
                                                &Js::dot_value(SIGNAL_PHONE_NUMBER_ERROR),
                                                &Js::empty_string(),
                                            ))
                                        })
                                })
                                .bind_error(SIGNAL_PHONE_NUMBER_ERROR)
                                .view()
                                .id("phone_number"),
                        ),
                )
                .child(
                    div().class("pt-3 w-full").child(
                        Button::default()
                            .label("Send code")
                            .color_primary()
                            .indicator(SIGNAL_IS_SUBMITTING)
                            .view()
                            .id("send_code")
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
            e.data_bind(SIGNAL_COUNTRY_CODE).data_on(|d| {
                d.change()
                    .js(&Js::assign(&Js::dot_value(SIGNAL_COUNTRY_CODE_ERROR), ""))
            })
        })
        .options(options)
        .bind_error(SIGNAL_COUNTRY_CODE_ERROR)
        .select_id("country_code_select")
        .view()
        .id("country_code")
}

fn view_screen_enter_code(phone_number: &str) -> Elem {
    form()
        .class("w-full h-full flex flex-col")
        .data_signal(SIGNAL_CODE, "''")
        .data_signal(SIGNAL_CODE_ERROR, "''")
        .child_signals_json(false)
        .data_on(|b| {
            b.submit().prevent_default().sse(
                &Route::ClickedVerifyCode {
                    phone_number: phone_number.to_string(),
                }
                .url(),
            )
        })
        .id("screen-enter-code")
        .data_indicator(SIGNAL_IS_SUBMITTING)
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
                            e.data_bind(SIGNAL_CODE).type_("tel").data_on(|d| {
                                d.input().js(&Js::assign(
                                    &Js::dot_value(SIGNAL_CODE_ERROR),
                                    &Js::empty_string(),
                                ))
                            })
                        })
                        .bind_error(SIGNAL_CODE_ERROR)
                        .view()
                        .id("code"),
                )
                .child(
                    div().class("pt-3 w-full").child(
                        Button::default()
                            .label("Verify code")
                            .color_primary()
                            .indicator(SIGNAL_IS_SUBMITTING)
                            .view()
                            .id("verify-code")
                            .class("w-full")
                            .type_("submit"),
                    ),
                ),
        )
}
