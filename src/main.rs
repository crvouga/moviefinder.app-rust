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
mod req;
mod respond;
mod route;
mod ui;
mod user_session;

#[tokio::main]
async fn main() {
    let env = env::Env::load().unwrap();

    let address = format!("0.0.0.0:{}", env.port);

    println!("LOG Starting server on http://{}", address);

    let ctx = Arc::new(
        ctx::Ctx::new(env.tmdb_api_read_access_token, env.database_url)
            .await
            .unwrap(),
    );

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
        form_data: http_request.form_data,
        session_id: session_id.clone(),
    };

    let res = respond::respond(&ctx, &req, &route).await.map_html(|html| {
        if http_request.headers.contains_key("hx-request") {
            html
        } else {
            Root::new().children(vec![html]).view()
        }
    });

    println!("LOG REQUEST {} {:?} {:?}", http_request.method, route, req);
    res.into()
}
