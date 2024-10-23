use std::sync::Arc;

mod account;
mod core;
mod ctx;
mod env;
mod feed;
mod key_value_db;
mod media;
mod respond;
mod route;
mod ui;

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

    let ctx = Arc::new(ctx::Ctx::new(env.tmdb_api_read_access_token));

    let started = core::http::server::start(&address, move |req| {
        let ctx_arc = Arc::clone(&ctx);
        respond(req, ctx_arc)
    })
    .await;

    match started {
        Ok(_) => println!("Server listening on http://{}", address),
        Err(err) => eprintln!("Errored while starting server: {}", err),
    }
}

async fn respond(http_req: core::http::Request, ctx: Arc<ctx::Ctx>) -> core::http::Response {
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
