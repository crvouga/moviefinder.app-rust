use core::{
    http::{request::Request, response_writer::ResponseWriter, server_sent_event::sse},
    mime_type::mime_type,
};
use ctx::Ctx;
use env::Env;
use req::Req;
use route::Route;
use std::sync::Arc;
use ui::root;

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

    let ctx = Arc::new(Ctx::new(&env).await.unwrap());

    log_info!(ctx.logger, "Starting server on http://{}", address);

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

    let r = Req {
        params: request.datastar_params(),
        session_id: request.session_id(),
    };

    log_info!(
        ctx.logger,
        "{:?} session_id={:?} params={:?}",
        maybe_route,
        r.session_id,
        r.params
    );

    let result = match (maybe_route, request.is_datastar_request()) {
        (Some(route), true) => respond::respond(&ctx, &r, &route, &mut w).await,

        (Some(route), false) => response_root(route, &request, &mut w).await,

        (None, true) => respond_fallback(&ctx, &r, &mut w).await,

        (None, false) => match resolve_public_asset(&request.url.path).await {
            Some(file_path) => response_public(&file_path, &request, &mut w).await,
            None => response_root(Route::Feed(feed::route::Route::Default), &request, &mut w).await,
        },
    };

    w.end().await?;

    result
}

async fn response_root(
    route: Route,
    r: &Request,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    let html: &String = &root::Root::new(route).view().render_with_doctype();

    w.content("text/html", r.to_accept_encoding(), html.as_bytes())
        .await
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
    r: &Request,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    if let Ok(mut file) = tokio::fs::File::open(file_path).await {
        let mut buffer = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(&mut file, &mut buffer).await?;
        let content_type = mime_type::from_path(file_path);
        // w.set_long_term_cache();
        w.content(content_type, r.to_accept_encoding(), &buffer)
            .await
    } else {
        w.write_body("404".as_bytes()).await
    }
}

async fn respond_fallback(
    ctx: &Ctx,
    r: &Req,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    let fallback = feed::route::Route::Default;
    sse()
        .event_execute_script()
        .data_script_push_url(&Route::Feed(fallback).encode())
        .send(w)
        .await?;
    feed::respond::respond(&ctx, r, &feed::route::Route::Default, w).await
}
