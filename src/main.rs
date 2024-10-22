use std::sync::Arc;

mod account;
mod core;
mod ctx;
mod env;
mod feed;
mod media;
mod respond;
mod route;
mod ui;

#[tokio::main]
async fn main() {
    let env = env::Env::load();

    let address = format!("0.0.0.0:{}", env.port);

    println!("Server listening on http://{}", address);

    let ctx = Arc::new(ctx::Ctx::new(env.tmdb_api_read_access_token));

    core::http::server::start(&address, move |req| {
        let ctx_arc = Arc::clone(&ctx);
        respond(req, ctx_arc)
    })
    .await;
}

async fn respond(http_req: core::http::Request, ctx: Arc<ctx::Ctx>) -> core::http::Response {
    let route = route::Route::decode(&http_req.path);

    println!("{} {:?}", http_req.method, route);

    let http_response = respond::respond(&route, &ctx)
        .await
        .map_html(|html| {
            if is_hx_request(&http_req) {
                html
            } else {
                ui::root::view_root(&[html])
            }
        })
        .into();

    http_response
}

fn is_hx_request(req: &core::http::Request) -> bool {
    req.headers.get("hx-request").is_some()
}
