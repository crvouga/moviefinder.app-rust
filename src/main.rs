mod feed;
mod http;
mod res;
mod respond;
mod route;

fn respond(req: http::Request) -> http::Response {
    let route = route::decode(req.path);

    println!("{} {:?}", req.method, route);

    let response = respond::respond(route);

    let http_respond = res::to_http_response(response);

    println!("{:?}", http_respond);

    return http_respond;
}

fn main() {
    let port = 8080;
    let address = "127.0.0.1:".to_owned() + &port.to_string();
    println!("Listening on http://localhost:{}", port);
    http::start_server(&address, respond);
}
