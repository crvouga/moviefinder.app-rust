use core::{
    http::{
        request::Request, response_writer::ResponseWriter, server_sent_event::sse,
        set_header::SetHeader,
    },
    mime_type::mime_type,
};
use ctx::Ctx;
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
    let route = r.route();

    log_info!(
        ctx.logger,
        "{:?} session_id={:?} params={:?}",
        route,
        r.session_id(),
        r.params()
    );

    match (route, r.is_datastar_request()) {
        (Some(route), true) => respond::respond(&ctx, &r, &route, &mut w).await,

        (Some(route), false) => response_root(route, &mut w).await,

        (None, true) => respond_fallback(&ctx, &r, &mut w).await,

        (None, false) => match resolve_public_asset(&r.url.path).await {
            Some(file_path) => response_public(&file_path, &mut w).await,
            None => response_root(Route::Feed(feed::route::Route::Default), &mut w).await,
        },
    }
}

async fn response_root(route: Route, w: &mut ResponseWriter) -> Result<(), std::io::Error> {
    let html: &String = &root::Root::new(route).view().render();
    w.html(html).await
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

async fn response_public(file_path: &str, w: &mut ResponseWriter) -> Result<(), std::io::Error> {
    if let Ok(mut file) = tokio::fs::File::open(file_path).await {
        let mut buffer = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(&mut file, &mut buffer).await?;

        let mime_type = mime_type::from_path(file_path);
        w.set_header("Content-Type", mime_type);

        w.write_body(&buffer).await
    } else {
        w.write_body("404".as_bytes()).await
    }
}

async fn respond_fallback(
    ctx: &Ctx,
    r: &Request,
    w: &mut ResponseWriter,
) -> Result<(), std::io::Error> {
    let fallback = feed::route::Route::Default;
    sse()
        .event_execute_script()
        .data_script_push_url(&Route::Feed(fallback).encode())
        .send(w)
        .await?;
    feed::respond::respond(&ctx.feed, r, &feed::route::Route::Default, w).await
}
