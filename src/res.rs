use std::collections::HashMap;

use crate::core::html::Elem;
use crate::core::htmx::hx::HxHeaders;
use crate::core::http::header::Header;
use crate::core::http::response::HttpResponse;

#[derive(Debug, Default, Clone)]
pub struct Res {
    pub variant: ResVariant,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Default, Clone)]
pub enum ResVariant {
    #[default]
    Empty,
    Html(Elem),
    Redirect {
        location: String,
        target: String,
    },
    Content {
        content_type: String,
        body: Vec<u8>,
    },
}

impl Res {
    pub fn html(body: Elem) -> Self {
        Self {
            variant: ResVariant::Html(body),
            ..Res::default()
        }
    }

    pub fn map_html(mut self, f: impl FnOnce(Elem) -> Elem) -> Self {
        self.variant = match self.variant {
            ResVariant::Html(body) => ResVariant::Html(f(body)),
            other => other,
        };
        self
    }

    pub fn empty() -> Self {
        Self {
            variant: ResVariant::Empty,
            ..Res::default()
        }
    }

    pub fn content(content_type: &str, body: Vec<u8>) -> Self {
        Self {
            variant: ResVariant::Content {
                content_type: content_type.to_string(),
                body,
            },
            ..Res::default()
        }
    }

    pub fn redirect(location: String, target: String) -> Self {
        Self {
            variant: ResVariant::Redirect {
                location: location.to_string(),
                target: target.to_string(),
            },
            ..Res::default()
        }
    }

    pub fn to_http_response(self) -> HttpResponse {
        let mut http_response: HttpResponse = self.variant.into();

        http_response.headers.extend(self.headers);

        http_response
    }

    pub fn cache(self) -> Self {
        self.header("Cache-Control", "public, max-age=31536000, immutable")
    }

    pub fn no_cache(self) -> Self {
        self.header("Cache-Control", "no-store")
    }
}

impl Header for Res {
    fn header(&self, key: &str, value: &str) -> Self {
        let mut headers = self.headers.clone();
        headers.insert(key.to_string(), value.to_string());
        Res {
            headers,
            ..self.clone()
        }
    }
}

impl HxHeaders for Res {}

impl Elem {
    pub fn res(self) -> Res {
        Res::html(self)
    }
}

impl From<ResVariant> for HttpResponse {
    fn from(res_variant: ResVariant) -> Self {
        match res_variant {
            ResVariant::Empty => HttpResponse::new(204),

            ResVariant::Html(body) => HttpResponse::new(200).body(body.render().into_bytes()),

            ResVariant::Redirect { location, target } => {
                HttpResponse::new(302).hx_redirect(&location, &target)
            }

            ResVariant::Content { body, content_type } => HttpResponse::new(200)
                .body(body)
                .header("Content-Type", &content_type),
        }
    }
}
