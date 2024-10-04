use crate::http;

#[derive(Debug)]
pub enum Res {
    Html(String),
    Redirect(String),
}

pub fn ensure_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{}", path)
    }
}

pub fn to_http_response(res: Res) -> http::Response {
    match res {
        Res::Html(body) => http::Response::new(200, body, vec![]),

        Res::Redirect(location) => http::Response::new(
            302,
            "".to_owned(),
            vec![(String::from("Location"), ensure_leading_slash(&location))],
        ),
    }
}
