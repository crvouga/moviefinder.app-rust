use crate::http;

pub enum Res {
    Html(String),
    Redirect(String),
}

pub fn to_http_response(res: Res) -> http::Response {
    match res {
        Res::Html(body) => http::Response::new(200, body),

        Res::Redirect(location) => http::Response::new(302, location),
    }
}
