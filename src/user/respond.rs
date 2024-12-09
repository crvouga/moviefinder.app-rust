use super::{account_screen, edit_profile, login_with_sms, logout, route::Route};
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::AccountScreen => account_screen::respond(ctx, r, w, &r.user_id).await,
        Route::LoginWithSms(child) => login_with_sms::respond::respond(ctx, r, child, w).await,
        Route::Logout(child) => logout::respond::respond(ctx, r, child, w).await,
        Route::EditProfile(child) => edit_profile::respond::respond(ctx, r, child, w).await,
    }
}
