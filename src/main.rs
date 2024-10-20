mod account;
mod app;
mod feed;
mod html;
mod http;
mod hx;
mod res;
mod respond;
mod route;
mod ui;

async fn respond(req: http::Request) -> http::Response {
    let route = route::decode(&req.path);

    println!("{} {:?}", req.method, route);

    let is_hx_request = req
        .headers
        .iter()
        .any(|(key, _value)| key.to_ascii_lowercase() == "hx-request");

    if is_hx_request {
        let response = respond::respond(route);

        let http_response = res::to_http_response(response);

        return http_response;
    }

    let html = app::root::view_root(&route).render();

    let response = res::Res::Html(html);

    let http_response = res::to_http_response(response);

    return http_response;
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = "0.0.0.0:".to_owned() + &port.to_string();
    println!("Listening on http://0.0.0.0:{}", port);
    http::start_server(&address, respond).await;
}
