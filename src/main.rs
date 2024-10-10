use html_builder::{Buffer, Html5 as _};
use std::fmt::Write;

mod feed;
mod http;
mod res;
mod respond;
mod route;

fn generate_html() -> Result<String, std::fmt::Error> {
    let mut buf = Buffer::new();
    let mut html = buf.html().attr("lang='en'");
    writeln!(html.head().title(), "Title!")?;
    writeln!(html.body().h1(), "Header!")?;
    return Ok(buf.finish());
}

fn respond(req: http::Request) -> http::Response {
    let route = route::decode(req.path);

    println!("{} {:?}", req.method, route);

    // let response = respond::respond(route);
    let html = generate_html().unwrap_or("<div>error</div>".to_owned());

    let response = res::Res::Html(html);

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
