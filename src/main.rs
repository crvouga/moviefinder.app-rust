use core::http::{request::HttpRequest, response::HttpResponse};
use req::Req;
use std::sync::Arc;
use ui::root::Root;
use user_session::{session_id::SessionId, wrap_session_id::wrap_session_id};

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
mod respond;
mod route;
mod ui;
mod user_session;

#[tokio::main]
async fn main() {
    let env = env::Env::load().unwrap();

    let address = format!("0.0.0.0:{}", env.port);

    let ctx = Arc::new(ctx::Ctx::new(env).await.unwrap());

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
    let route = route::Route::decode(&http_request.path);

    let req = Req {
        session_id,
        form_data: http_request.form_data.clone(),
    };

    log_info!(ctx.logger, "{:?} {:?}", route, req);

    let res = respond::respond(&ctx, &req, &route).await;

    let is_hx_request = http_request.headers.contains_key("hx-request");

    let res_with_root = if is_hx_request {
        res
    } else {
        res.no_cache()
            .map_html(|html| Root::new().children(vec![html]).view())
    };

    let mut http_response: HttpResponse = res_with_root.into();

    http_response.compress(&http_request);

    http_response
}
