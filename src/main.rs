use core::{
    http::{request::Request, response_writer::ResponseWriter},
    js::Js,
    mime_type::mime_type,
};
use ctx::Ctx;
use env::Env;
use feed::feed_screen;
use req::Req;
use std::sync::Arc;
use ui::{root, route::AppRoute};

mod core;
mod ctx;
mod env;
mod feed;
mod fixture;
mod list;
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

    info!(ctx.log, "Server listening here http://{}", address);

    core::http::server::start(&address, move |r, w| respond(ctx.clone(), r, w))
        .await
        .unwrap();
}

async fn respond(
    ctx: Arc<Ctx>,
    request: Request,
    mut w: ResponseWriter,
) -> Result<(), crate::core::error::CoreError> {
    request.write_session_id_cookie(&mut w);

    let maybe_route = request.route();

    debug!(ctx.log, "Request {:?}", maybe_route);

    let session_id = request.session_id();

    let r = Req {
        payload: request.datastar_payload(),
        session_id,
        url: request.url.path.clone(),
    };

    if let None = maybe_route {
        info!(ctx.log, "No route found for {:?}", r.url);
    }

    info!(
        ctx.log,
        "{:?} session_id={:?} payload={:?}", maybe_route, r.session_id, r.payload
    );

    let result: Result<(), crate::core::error::CoreError> =
        match (maybe_route, request.is_datastar_request()) {
            (Some(route), true) => respond::respond(&ctx, &r, &route, &mut w).await,

            (Some(route), false) => response_root(&request, &mut w, route.url()).await,

            (None, true) => respond_fallback(&ctx, &r, &mut w).await,

            (None, false) => match resolve_public_asset(&r.url).await {
                Some(file_path) => response_public(&file_path, &request, &mut w).await,
                None => {
                    response_root(
                        &request,
                        &mut w,
                        feed_screen::route::Route::FeedScreenDefault.url(),
                    )
                    .await
                }
            },
        };

    match &result {
        Ok(_) => {}
        Err(e) => {
            let error_message = format!("Error: {:?}", e);

            w.send_script(&Js::console_error(&Js::quote(&error_message)))
                .await?;
        }
    }

    w.end().await?;

    result
}

async fn response_root(
    _r: &Request,
    w: &mut ResponseWriter,
    url: String,
) -> Result<(), crate::core::error::CoreError> {
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
) -> Result<(), crate::core::error::CoreError> {
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
) -> Result<(), crate::core::error::CoreError> {
    w.send_script(&Js::push_url(
        &feed_screen::route::Route::FeedScreenDefault.url(),
    ))
    .await?;

    feed::respond::respond(
        &ctx,
        r,
        &feed::route::Route::FeedScreen(feed_screen::route::Route::FeedScreenDefault),
        w,
    )
    .await
}
