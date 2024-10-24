use core::http::{
    request::HttpRequest,
    response::HttpResponse,
    response_cookie::{HttpResponseCookie, SameSite},
};
use std::sync::Arc;

mod account;
mod core;
mod ctx;
mod env;
mod feed;
mod fixture;
mod key_value_db;
mod media;
mod respond;
mod route;
mod ui;
mod user_session;

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

    let session_cookie = HttpResponseCookie {
        domain: None,
        expires: None,
        path: None,
        http_only: true,
        secure: false,
        max_age: None,
        name: "movie_finder_app_session_id".to_string(),
        value: "".to_string(),
        same_site: Some(SameSite::Lax),
    };

    println!("Session cookie: {}", session_cookie);

    let started = core::http::server::start(&address, move |req| {
        let ctx_arc = Arc::clone(&ctx);
        respond(req, ctx_arc)
    })
    .await;

    if let Err(err) = started {
        eprintln!("Errored while starting server: {}", err);
        return;
    }
}

async fn respond(http_req: HttpRequest, ctx: Arc<ctx::Ctx>) -> HttpResponse {
    let route = route::Route::decode(&http_req.path);

    println!("{} {:?}", http_req.method, route);

    respond::respond(&route, &ctx)
        .await
        .map_html(|html| {
            if http_req.headers.contains_key("hx-request") {
                html
            } else {
                ui::root::view_root(&[html])
            }
        })
        .into()
}
