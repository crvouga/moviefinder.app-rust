use core::{
    http::{request::HttpRequest, response::HttpResponse},
    session::{session_id::SessionId, wrap_session_id::wrap_session_id},
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

    core::http::server::start(
        &address,
        wrap_session_id(move |session_id, http_request| {
            let ctx_arc = Arc::clone(&ctx);

            respond(http_request, session_id, ctx_arc)
        }),
    )
    .await
    .unwrap();
}

async fn respond(
    http_request: HttpRequest,
    session_id: SessionId,
    ctx: Arc<ctx::Ctx>,
) -> HttpResponse {
    let route = route::Route::decode(&http_request.url.path);

    let req = Req {
        session_id,
        params: http_request.to_params(),
    };

    log_info!(ctx.logger, "{:?} {:?}", route, req);

    let res = respond::respond(&ctx, &req, &route).await;

    let res_with_root = if http_request.headers.contains_key("hx-request") {
        res
    } else if matches!(res.variant, ResVariant::Html(_)) {
        res.no_cache()
            .map_html(|html| Root::new().children(vec![html]).view())
    } else {
        res
    };

    let mut http_response: HttpResponse = res_with_root.into();

    http_response.compress_gzip();

    http_response
}
