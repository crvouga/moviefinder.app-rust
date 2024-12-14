use core::{
    datastar::datastar::js_console_error,
    http::{request::Request, response_writer::ResponseWriter},
    mime_type::mime_type,
};
use ctx::Ctx;
use env::Env;
use req::Req;
use route::Route;
use std::sync::Arc;
use ui::{root, route::Routable};

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
mod user;

#[tokio::main]
async fn main() {
    let env = Env::load();

    let address = format!("0.0.0.0:{}", env.port);

    let ctx = Arc::new(Ctx::new(&env).await);

    info!(ctx.logger, "Server listening here http://{}", address);

    core::http::server::start(&address, move |r, w| respond(ctx.clone(), r, w))
        .await
        .unwrap();
}

async fn respond(
    ctx: Arc<Ctx>,
    request: Request,
    mut w: ResponseWriter,
) -> Result<(), std::io::Error> {
    request.write_session_id_cookie(&mut w);

    let maybe_route = request.route();

    debug!(ctx.logger, "Request {:?}", maybe_route);

    let session_id = request.session_id();

    let r = Req {
        payload: request.datastar_params(),
        session_id: session_id,
    };

    if let None = maybe_route {
        info!(ctx.logger, "No route found for {:?}", request.url.path);
    }

    info!(
        ctx.logger,
        "{:?} session_id={:?} payload={:?}", maybe_route, r.session_id, r.payload
    );

    let result: Result<(), std::io::Error> = match (maybe_route, request.is_datastar_request()) {
        (Some(route), true) => respond::respond(&ctx, &r, &route, &mut w).await,

        (Some(route), false) => response_root(&request, &mut w, route.url()).await,

        (None, true) => respond_fallback(&ctx, &r, &mut w).await,

        (None, false) => match resolve_public_asset(&request.url.path).await {
            Some(file_path) => response_public(&file_path, &request, &mut w).await,
            None => {
                response_root(
                    &request,
                    &mut w,
                    feed::route::Route::FeedScreenDefault.url(),
                )
                .await
            }
        },
    };

    match &result {
        Ok(_) => {}
        Err(e) => {
            let error_message = format!("Error: {:?}", e);

            // error!(ctx.logger, "{}", error_message);

            w.send_script(&js_console_error(&error_message)).await?;
        }
    }

    w.end().await?;

    result
}

async fn response_root(
    _r: &Request,
    w: &mut ResponseWriter,
    url: String,
) -> Result<(), std::io::Error> {
    let html: &String = &root::Root::new(url).view().render_with_doctype();

    w.content("text/html", html.as_bytes()).await
}

async fn resolve_public_asset(path: &str) -> Option<String> {
    let public_dir = "public";

    let trimmed_path = path.trim_start_matches('/');

    if trimmed_path.is_empty() {
        return None;
    }

    let full_path = format!("{}/{}", public_dir, trimmed_path);

    if tokio::fs::metadata(&full_path).await.is_ok() {
        return Some(full_path);
    }

    None
}

async fn response_public(
    file_path: &str,
    _r: &Request,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    if let Ok(mut file) = tokio::fs::File::open(file_path).await {
        let mut buffer = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(&mut file, &mut buffer).await?;
        let content_type = mime_type::from_path(file_path);
        w.content(content_type, &buffer).await
    } else {
        w.write_body("404".as_bytes()).await
    }
}

async fn respond_fallback(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    let fallback = feed::route::Route::FeedScreenDefault;
    w.send_push_url(&Route::Feed(fallback).url()).await?;

    feed::respond::respond(&ctx, r, &feed::route::Route::FeedScreenDefault, w).await
}
