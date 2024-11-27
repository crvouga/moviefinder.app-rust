use core::http::{request::Request, response_writer::ResponseWriter, set_header::SetHeader};
use env::Env;
use req::Req;
use std::sync::Arc;
use ui::root::{self};

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

#[tokio::main]
async fn main() {
    let env = Env::load();

    let address = format!("0.0.0.0:{}", env.port);

    let ctx = Arc::new(ctx::Ctx::new(&env).await.unwrap());

    log_info!(ctx.logger, "Starting server on http://{}", address);

    core::http::server::start(&address, move |r, mut w| {
        let ctx_arc = Arc::clone(&ctx);
        r.write_session_id_cookie(&mut w);
        respond(ctx_arc, r, w)
    })
    .await
    .unwrap();
}

async fn respond(
    ctx: Arc<ctx::Ctx>,
    r: Request,
    mut w: ResponseWriter,
) -> Result<(), std::io::Error> {
    let route: route::Route = route::Route::decode(&r.url.path);

    if is_html_request(&r) && !is_fragment_request(&r) {
        let html = &root::Root::new(route).view().render();
        return w.html(html).await;
    }

    let req = Req {
        session_id: r.session_id(),
        params: r.params(),
    };

    log_info!(ctx.logger, "{:?} {:?}", route, req);

    respond::respond(&ctx, &req, &route, &mut w).await;

    Ok(())
}

fn is_html_request(request: &Request) -> bool {
    request
        .headers
        .get("accept")
        .unwrap_or(&"".to_string())
        .contains("html")
}

fn is_fragment_request(request: &Request) -> bool {
    request.headers.contains_key("hx-request") || request.headers.contains_key("datastar-request")
}
