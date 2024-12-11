use super::route::Route;
use crate::{
    core::{
        html::{div, form, Elem},
        http::response_writer::ResponseWriter,
        ui::{avatar::Avatar, spinner_page, text_field::TextField, top_bar::TopBar},
    },
    ctx::Ctx,
    req::Req,
    ui::{bottom_bar_form::BottomBarForm, route::Routable},
    user::{self, user_profile::user_profile_::UserProfile},
};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::Screen { .. } => {
            w.send_screen(view_screen_loading()).await?;

            let user_id = match r.user_id.clone() {
                Some(user_id) => user_id,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            let maybe_profile = ctx.user_profile_db.find_one_by_user_id(&user_id).await?;

            let profile = match maybe_profile {
                Some(profile) => profile,
                None => return respond_failed_to_load(ctx, r, w).await,
            };

            w.send_screen(view_screen_loaded(profile)).await?;

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
    div().child(
        TopBar::default()
            .back_url(user::route::Route::AccountScreen.url())
            .title("Edit Profile")
            .view(),
    )
}

fn view_screen_loading() -> Elem {
    view_screen_root().child(spinner_page::view())
}

fn view_screen_loaded(profile: UserProfile) -> Elem {
    view_screen_root()
        .child(
            form()
                .child(
                    TextField::default()
                        .label("Username")
                        .map_input(|i| i.data_bind("username"))
                        .placeholder("Username")
                        .view(),
                )
                .child(
                    div().child(Avatar::default().src(" ").view()).child(
                        TextField::default()
                            .label("Avatar Seed")
                            .map_input(|i| i.data_bind("avatar_seed"))
                            .placeholder("Avatar Seed")
                            .view(),
                    ),
                ),
        )
        .child(
            BottomBarForm::default()
                .cancel_url(&user::route::Route::AccountScreen.url())
                .save_url(
                    &(Route::ClickedSave {
                        user_id: profile.user_id.clone(),
                    })
                    .url(),
                )
                .view(),
        )
}
