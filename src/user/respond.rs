use super::{account_screen, login_with_sms, logout, route::Route};
use crate::{core::http::response_writer::ResponseWriter, ctx::Ctx, req::Req};

pub async fn respond(
    ctx: &Ctx,
    r: &Req,
    route: &Route,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    match route {
        Route::LoginWithSms(child) => login_with_sms::respond::respond(ctx, r, child, w).await,
        Route::Logout(child) => logout::respond::respond(ctx, r, child, w).await,
        Route::AccountScreen => account_screen::respond(ctx, r, w, &r.user_id).await,
    }
}
