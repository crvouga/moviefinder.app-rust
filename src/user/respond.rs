use super::{account_screen, edit_profile, login, login_with_sms, logout, route::Route};
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), crate::core::error::CoreError> {
    match route {
        Route::AccountScreen(route) => account_screen::respond::respond(ctx, r, route, w).await,
        Route::LoginWithSms(route) => login_with_sms::respond::respond(ctx, r, route, w).await,
        Route::Logout(route) => logout::respond::respond(ctx, r, route, w).await,
        Route::EditProfile(route) => edit_profile::respond::respond(ctx, r, route, w).await,
        Route::Login(route) => login::respond::respond(ctx, r, route, w).await,
    }
}
