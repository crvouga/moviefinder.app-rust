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
    core::env::load().unwrap_or_default();

    let port = core::env::read("PORT").unwrap_or("8080".to_string());

    let address = format!("0.0.0.0:{}", port);

    println!("Listening on http://0.0.0.0:{}", port);

    let tmdb_api_read_access_token =
        core::env::read("TMDB_API_READ_ACCESS_TOKEN").unwrap_or_default();

    let ctx = Arc::new(ctx::Ctx::new(tmdb_api_read_access_token));

    http::server::start(&address, move |req| {
        let ctx_arc = Arc::clone(&ctx);
        respond(req, ctx_arc)
    })
    .await;
}

fn is_hx_request(req: &http::Request) -> bool {
    req.headers.get("HX-Request").is_some()
}

async fn respond(http_req: http::Request, ctx: Arc<ctx::Ctx>) -> http::Response {
    let route = route::decode(&http_req.path);

    println!("{} {:?}", http_req.method, route);

    let res = respond::respond(route, &ctx).await.map_html(|html| {
        if is_hx_request(&http_req) {
            html
        } else {
            app::root::view_root(&[html])
        }
    });

    return res.into();
}
