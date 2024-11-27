use core::http::{request::Request, response_writer::ResponseWriter};
use env::Env;
use route::Route;
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
    if is_html_request(&r) && !is_fragment_request(&r) {
        let html = &root::Root::new(r.route()).view().render();
        return w.html(html).await;
    }

    log_info!(
        ctx.logger,
        "{:?} session_id={:?} params={:?}",
        r.route(),
        r.session_id(),
        r.params()
    );

    respond::respond(&ctx, &r, &r.route(), &mut w).await
}

impl Request {
    pub fn route(&self) -> Route {
        Route::decode(&self.url.path)
    }
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
