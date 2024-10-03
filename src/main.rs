mod feed;
mod http;
mod res;
mod respond;
mod route;

fn handle_request(req: http::Request) -> http::Response {
    println!("{} {}", req.method, req.path);

    let decoded = route::decode(req.path);

    let response = respond::respond(decoded);

    let html_response = res::to_http_response(response);

    return html_response;
}

fn main() {
    let port = 8080;
    let address = "127.0.0.1:".to_owned() + &port.to_string();
    println!("Listening on http://localhost:{}", port);
    http::start_server(&address, handle_request);
}
