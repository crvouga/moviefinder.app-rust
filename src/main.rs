use core::{
    http::{request::HttpRequest, response_writer::HttpResponseWriter},
    session::{session_id::SessionId, session_id_cookie::read_write_session_id_cookie},
};
use env::Env;
use req::Req;
use res::{Res, ResVariant};
use std::sync::Arc;
use ui::root::Root;

mod account;
mod core;
mod ctx;
mod env;
mod feed;
mod fixture;
mod key_value_db;
mod media;
mod person;
mod req;
mod res;
mod respond;
mod route;
mod ui;

#[tokio::main]
async fn main() {
    let env = Env::load();

    let address = format!("0.0.0.0:{}", env.port);

    let ctx = Arc::new(ctx::Ctx::new(&env).await.unwrap());

    log_info!(ctx.logger, "Starting server on http://{}", address);

    core::http::server::start(&address, move |request, mut response_writer| {
        let ctx_arc = Arc::clone(&ctx);
        let session_id = read_write_session_id_cookie(&request, &mut response_writer);
        respond(ctx_arc, session_id, request, response_writer)
    })
    .await
    .unwrap();
}

async fn respond(
    ctx: Arc<ctx::Ctx>,
    session_id: SessionId,
    request: HttpRequest,
    mut response_writer: HttpResponseWriter,
) -> Result<(), std::io::Error> {
    let route: route::Route = route::Route::decode(&request.url.path);

    let req = Req {
        session_id,
        params: request.to_params(),
    };

    log_info!(ctx.logger, "{:?} {:?}", route, req);

    let res = respond::respond(&ctx, &req, &route).await;

    let wrapped_res = if should_wrap_with_root(&res, &request) {
        res.no_cache()
            .map_html(|html| Root::new().children(vec![html]).view())
    } else {
        res
    };

    wrapped_res
        .write_http_response(&mut response_writer)
        .await?;

    response_writer.end().await
}

fn should_wrap_with_root(res: &Res, request: &HttpRequest) -> bool {
    !request.headers.contains_key("hx-request") && matches!(res.variant, ResVariant::Html(_))
}
