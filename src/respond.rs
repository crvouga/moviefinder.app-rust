use crate::account;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::req::Req;
use crate::res::Res;
use crate::route::Route;
use crate::ui::resizable_image;

pub async fn respond(ctx: &ctx::Ctx, req: &Req, route: &Route) -> Res {
    match route {
        Route::Feed(route) => feed::respond::respond(&ctx.feed, req, route).await,

        Route::Account(route) => account::respond::respond(route),

        Route::Media(route) => media::respond::respond(&ctx, route).await,

        Route::ResizableImage(route) => {
            resizable_image::respond::response(&ctx.resizable_image, route, req.clone()).await
        }

        Route::Favicon => Res::empty(),

        Route::RobotsTxt => Res::content("text", "User-agent: *\nDisallow:".as_bytes().to_vec()),

        Route::OutputCss => {
            Res::content("text/css", include_bytes!("./output.css").to_vec()).cache()
        }

        Route::Unknown(_route) => {
            feed::respond::respond(&ctx.feed, req, &feed::route::Route::Default).await
        }
    }
}
