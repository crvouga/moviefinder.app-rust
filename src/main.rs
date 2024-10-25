use core::http::{
    request::HttpRequest,
    response::HttpResponse,
    response_cookie::{HttpResponseCookie, SameSite},
};
use std::{collections::HashMap, sync::Arc};

use futures::FutureExt;
use req::Req;
use user_session::session_id::SessionId;

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

const SESSION_COOKIE_NAME: &str = "session_id";

#[tokio::main]
async fn main() {
    let env = match env::Env::load() {
        Some(env) => env,
        None => {
            eprintln!("Failed to load environment variables");
            return;
        }
    };

    let address = format!("0.0.0.0:{}", env.port);

    println!("Starting server on http://{}", address);

    let ctx = Arc::new(ctx::Ctx::new(env.tmdb_api_read_access_token));

    let started = core::http::server::start(&address, move |http_request| {
        let ctx_arc = Arc::clone(&ctx);

        let maybe_session_id = http_request
            .cookies
            .get(SESSION_COOKIE_NAME)
            .cloned()
            .and_then(SessionId::new)
            .clone();

        let session_id = maybe_session_id.clone().unwrap_or_default();

        respond(http_request, session_id, ctx_arc).map(move |mut http_response| {
            let session_id = maybe_session_id.clone().unwrap_or_default();
            if maybe_session_id.is_none() {
                let session_cookie = HttpResponseCookie {
                    domain: None,
                    expires: None,
                    path: None,
                    http_only: true,
                    secure: false,
                    max_age: None,
                    name: SESSION_COOKIE_NAME.to_string(),
                    value: session_id.as_str().to_string(),
                    same_site: Some(SameSite::Lax),
                };
                http_response
                    .headers
                    .insert("Set-Cookie".to_string(), session_cookie.to_string());
            }

            http_response
        })
    })
    .await;

    if let Err(err) = started {
        eprintln!("Errored while starting server: {}", err);
        return;
    }
}

async fn respond(
    http_request: HttpRequest,
    session_id: SessionId,
    ctx: Arc<ctx::Ctx>,
) -> HttpResponse {
    let route = route::Route::decode(&http_request.path);

    let req = Req {
        form_data: http_request.form_data,
        session_id,
    };

    println!("{} {:?} {:?}", http_request.method, route, req);

    respond::respond(&ctx, &req, &route)
        .await
        .map_html(|html| {
            if http_request.headers.contains_key("hx-request") {
                html
            } else {
                ui::root::view_root(&[html])
            }
        })
        .into()
}
