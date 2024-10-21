use std::sync::Arc;

mod account;
mod app;
mod core;
mod ctx;
mod env;
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
    let env = env::Env::load();

    let address = format!("0.0.0.0:{}", env.port);

    println!("Server listening on http://{}", address);

    let ctx = Arc::new(ctx::Ctx::new(env.tmdb_api_read_access_token));

    http::server::start(&address, move |req| {
        let ctx_arc = Arc::clone(&ctx);
        respond(req, ctx_arc)
    })
    .await;
}

async fn respond(http_req: http::Request, ctx: Arc<ctx::Ctx>) -> http::Response {
    let route = route::decode(&http_req.path);

    println!("{} {:?}", http_req.method, route);

    respond::respond(route, &ctx)
        .await
        .map_html(|html| {
            if is_hx_request(&http_req) {
                html
            } else {
                app::root::view_root(&[html])
            }
        })
        .into()
}

fn is_hx_request(req: &http::Request) -> bool {
    req.headers.get("hx-request").is_some()
}
