use std::collections::HashMap;

use crate::core::html::Elem;

use super::http;

#[derive(Debug)]
pub enum Res {
    Html(Elem),
    Redirect(String),
    Text(String),
    Empty,
}

impl Res {
    pub fn map_html(self, f: impl FnOnce(Elem) -> Elem) -> Res {
        match self {
            Res::Html(body) => Res::Html(f(body)),
            _ => self,
        }
    }
}

impl From<Elem> for Res {
    fn from(elem: Elem) -> Self {
        Res::Html(elem)
    }
}

impl From<Res> for http::Response {
    fn from(res: Res) -> Self {
        match res {
            Res::Html(body) => http::Response::new(200, body.render(), HashMap::new()),
            Res::Redirect(location) => {
                let mut headers = HashMap::new();
                headers.insert(
                    "Location".to_string().to_ascii_lowercase(),
                    ensure_leading_slash(&location),
                );
                http::Response::new(302, "".to_owned(), headers)
            }
            Res::Text(text) => http::Response::new(200, text, HashMap::new()),
            Res::Empty => http::Response::new(204, "".to_owned(), HashMap::new()),
        }
    }
}

fn ensure_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{}", path)
    }
}
