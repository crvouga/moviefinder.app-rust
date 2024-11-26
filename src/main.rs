use core::{
    http::{request::HttpRequest, response_writer::HttpResponseWriter},
    session::{session_id::SessionId, wrap_session_id_v2::write_session_id},
};
use env::Env;
use req::Req;
use res::ResVariant;
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

    core::http::server_v2::start(&address, move |request, mut response_writer| {
        let ctx_arc = Arc::clone(&ctx);
        let session_id = write_session_id(&request, &mut response_writer);
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

    let res_with_root = if request.headers.contains_key("hx-request") {
        res
    } else if matches!(res.variant, ResVariant::Html(_)) {
        res.no_cache()
            .map_html(|html| Root::new().children(vec![html]).view())
    } else {
        res
    };

    res_with_root
        .write_http_response(&mut response_writer)
        .await
}
