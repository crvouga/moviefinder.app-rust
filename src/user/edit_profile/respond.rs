use super::route::Route;
use crate::{
    core::{
        datastar::datastar::js_quote,
        html::{div, fieldset, form, Elem},
        http::response_writer::ResponseWriter,
        ui::{avatar::Avatar, spinner_page, text_field::TextField, top_bar::TopBar},
    },
    ctx::Ctx,
    req::Req,
    ui::{bottom_bar_form::BottomBarForm, route::Routable},
    user::{self, user_profile::user_profile_::UserProfile},
};

const SIGNAL_USERNAME: &str = "signal_username";
const SIGNAL_FULL_NAME: &str = "signal_full_name";
const SIGNAL_AVATAR_SEED: &str = "signal_avatar_seed";

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { .. } => {
            // w.send_screen(view_screen_loading()).await?;
            w.send_screen(view_screen_loaded(None)).await?;

            let user_id = match r.user_id.clone() {
                Some(user_id) => user_id,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            let maybe_profile = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

            let profile = match maybe_profile {
                Some(profile) => profile,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            w.send_screen(view_screen_loaded(Some(&profile))).await?;

            w.send_signal(
                SIGNAL_USERNAME,
                &js_quote(&profile.username.clone().to_string()),
            )
            .await?;

            w.send_signal(
                SIGNAL_AVATAR_SEED,
                &js_quote(&profile.avatar_seed.clone().unwrap_or_default()),
            )
            .await?;

            Ok(())
        }

        Route::CheckUsernameTaken { .. } => {
            unimplemented!()
        }

        Route::ClickedCancel { .. } => {
            unimplemented!()
        }

        Route::ClickedSave { .. } => {
            unimplemented!()
        }
    }
}

async fn respond_failed_to_load(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    w.send_toast_dark("User not found. Try logging in again")
        .await?;

    user::account_screen::respond(ctx, r, w, &None).await?;

    Ok(())
}

fn view_screen_root() -> Elem {
    div()
        .class("flex flex-col w-full flex-1 overflow-hidden")
        .data_signal(SIGNAL_USERNAME, "''")
        .data_signal(SIGNAL_AVATAR_SEED, "''")
        .data_signal(SIGNAL_FULL_NAME, "''")
        .debug_signals(false)
        .child(
            TopBar::default()
                .back_url(user::route::Route::AccountScreen.url())
                .title("Edit Profile")
                .view(),
        )
}

fn view_screen_loading() -> Elem {
    view_screen_root().child(spinner_page::view())
}

fn view_screen_loaded(profile: Option<&UserProfile>) -> Elem {
    view_screen_root()
        .child(
            form()
                .class("flex-1 w-full flex flex-col gap-12 p-6 overflow-y-scroll pb-36")
                .child(
                    fieldset()
                        .class("flex flex-col w-full gap-4 items-center justify-center")
                        .child(Avatar::default().src(" ").view().class("size-36"))
                        .child(
                            TextField::default()
                                .label("Avatar Seed")
                                .map_input(|i| i.data_bind(SIGNAL_AVATAR_SEED))
                                .placeholder("Avatar Seed")
                                .view(),
                        ),
                )
                .child(
                    fieldset().child(
                        TextField::default()
                            .label("Username")
                            .map_input(|i| i.data_bind(SIGNAL_USERNAME))
                            .placeholder("Username")
                            .view(),
                    ),
                )
                .child(
                    fieldset().child(
                        TextField::default()
                            .label("Full name")
                            .map_input(|i| i.data_bind(SIGNAL_FULL_NAME))
                            .placeholder("Full name")
                            .view(),
                    ),
                ),
        )
        .child(
            BottomBarForm::default()
                .cancel_url(&user::route::Route::AccountScreen.url())
                .save_url(&match profile {
                    Some(profile) => {
                        let route = Route::ClickedSave {
                            user_id: profile.user_id.clone(),
                        };

                        let url = route.url();

                        url
                    }
                    None => "".to_string(),
                })
                .view(),
        )
}
