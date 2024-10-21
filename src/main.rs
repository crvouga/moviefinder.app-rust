use dotenv::dotenv;
use std::sync::Arc;

mod account;
mod app;
mod core;
mod ctx;
mod feed;
mod html;
mod http;
mod hx;
mod media;
mod res;
mod respond;
mod route;
mod ui;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = "0.0.0.0:".to_owned() + &port.to_string();
    println!("Listening on http://0.0.0.0:{}", port);

    let tmdb_api_read_access_token =
        std::env::var("TMDB_API_READ_ACCESS_TOKEN").unwrap_or("".to_string());

    let ctx = Arc::new(ctx::Ctx::new(tmdb_api_read_access_token));

    http::server::start(&address, move |req| {
        let ctx_arc = Arc::clone(&ctx);
        respond(req, ctx_arc)
    })
    .await;
}

async fn respond(req: http::Request, ctx: Arc<ctx::Ctx>) -> http::Response {
    let route = route::decode(&req.path);

    println!("{} {:?}", req.method, route);

    let is_hx_request = req
        .headers
        .iter()
        .any(|(key, _value)| key.to_ascii_lowercase() == "hx-request");

    if is_hx_request {
        let response = respond::respond(route, &ctx).await;

        let http_response = res::to_http_response(response);

        return http_response;
    }

    let html = app::root::view_root(&route).render();

    let response = res::Res::Html(html);

    let http_response = res::to_http_response(response);

    return http_response;
}
