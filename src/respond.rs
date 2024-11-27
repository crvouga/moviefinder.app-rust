use crate::account;
use crate::core::http::response_writer::HttpResponseWriter;
use crate::ctx;
use crate::feed;
use crate::media;
use crate::req::Req;
use crate::res::Res;
use crate::route::Route;
use crate::ui::resizable_image;

pub async fn respond(
    response_writer: &mut HttpResponseWriter,
    ctx: &ctx::Ctx,
    req: &Req,
    route: &Route,
) -> Res {
    match route {
        Route::Feed(route) => feed::respond::respond(response_writer, &ctx.feed, req, route).await,

        Route::Account(route) => account::respond::respond(response_writer, route).await,

        Route::Media(route) => media::respond::respond(response_writer, &ctx, route).await,

        Route::ResizableImage(route) => {
            resizable_image::respond::response(&ctx.resizable_image, route, req.clone()).await
        }

        Route::Favicon => Res::empty(),

        Route::RobotsTxt => Res::content("text", "User-agent: *\nDisallow:".as_bytes().to_vec()),

        Route::OutputCss => {
            response_writer.css(include_bytes!("./output.css")).await;
            Res::empty()
        }

        Route::Unknown(_route) => {
            account::respond::respond(response_writer, &account::route::Route::Index).await
        }
    }
}
