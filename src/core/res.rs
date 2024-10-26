use super::http::response::HttpResponse;
use crate::core::html::Elem;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Res {
    variant: ResVariant,
    actions: Vec<ResAction>,
}

#[derive(Debug)]
pub enum ResAction {
    PushUrl(String),
}

#[derive(Debug)]
pub enum ResVariant {
    Html(Elem),
    Redirect(String),
    Text(String),
    Empty,
}

impl Res {
    pub fn html(body: Elem) -> Self {
        Res {
            variant: ResVariant::Html(body),
            ..Res::default()
        }
    }

    pub fn empty() -> Self {
        Res {
            variant: ResVariant::Empty,
            ..Res::default()
        }
    }

    pub fn text(text: &str) -> Self {
        Res {
            variant: ResVariant::Text(text.to_owned()),
            ..Res::default()
        }
    }

    pub fn redirect(location: String) -> Self {
        Res {
            variant: ResVariant::Redirect(location),
            ..Res::default()
        }
    }

    pub fn map_html(self, f: impl FnOnce(Elem) -> Elem) -> Res {
        match self.variant {
            ResVariant::Html(body) => Res::html(f(body)),
            _ => self,
        }
    }

    pub fn push_url(self, _url: &str) -> Self {
        Self {
            actions: self
                .actions
                .into_iter()
                .chain(vec![ResAction::PushUrl(_url.to_owned())])
                .collect(),
            ..self
        }
    }
}

impl Default for Res {
    fn default() -> Self {
        Res {
            variant: ResVariant::Empty,
            actions: Vec::new(),
        }
    }
}

impl From<Elem> for Res {
    fn from(elem: Elem) -> Self {
        Res::html(elem)
    }
}

impl From<Res> for HttpResponse {
    fn from(res: Res) -> Self {
        let mut http_response: HttpResponse = res.variant.into();

        for action in res.actions {
            match action {
                ResAction::PushUrl(url) => {
                    http_response.headers.insert(
                        "X-Push-Url".to_string().to_ascii_lowercase(),
                        ensure_leading_slash(&url),
                    );
                }
            }
        }

        http_response
    }
}

impl From<ResVariant> for HttpResponse {
    fn from(res_variant: ResVariant) -> Self {
        match res_variant {
            ResVariant::Html(body) => HttpResponse::new(200, body.render(), HashMap::new()),
            ResVariant::Redirect(location) => {
                let mut headers = HashMap::new();
                headers.insert(
                    "Location".to_string().to_ascii_lowercase(),
                    ensure_leading_slash(&location),
                );
                HttpResponse::new(302, "".to_owned(), headers)
            }
            ResVariant::Text(text) => HttpResponse::new(200, text, HashMap::new()),
            ResVariant::Empty => HttpResponse::new(204, "".to_owned(), HashMap::new()),
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
