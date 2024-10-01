mod http;
mod res;
mod respond;
mod route;

fn handle_request(req: http::Request) -> http::Response {
    println!("{} {}", req.method, req.path);

    let decoded = route::decode(req.path);

    let response = respond::respond(decoded);

    let html_response = res::to_http_response(response);

    html_response
}

fn main() {
    http::start_server("127.0.0.1:8080", handle_request);
}
