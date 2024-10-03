use crate::http;

pub enum Res {
    Html(String),
    Redirect(String),
}

pub fn to_http_response(res: Res) -> http::Response {
    match res {
        Res::Html(body) => http::Response::new(200, body, vec![]),

        Res::Redirect(location) => http::Response::new(
            302,
            "".to_owned(),
            vec![(String::from("Location"), location)],
        ),
    }
}
