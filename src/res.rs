use std::collections::HashMap;

use crate::core::html::Elem;
use crate::core::htmx::hx::HxHeaders;
use crate::core::http::header::SetHeader;
use crate::core::http::response_writer::HttpResponseWriter;

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

    pub fn cache(mut self) -> Self {
        self.set_header("Cache-Control", "public, max-age=31536000, immutable");
        self
    }

    pub fn no_cache(mut self) -> Self {
        self.set_header("Cache-Control", "no-store");
        self
    }

    pub async fn write_http_response(
        self,
        response_writer: &mut HttpResponseWriter,
    ) -> Result<(), std::io::Error> {
        for (key, value) in self.headers {
            response_writer.set_header(&key, &value);
        }
        match self.variant {
            ResVariant::Empty => Ok(()),

            ResVariant::Html(elem) => {
                response_writer
                    .write_body(&elem.render().into_bytes())
                    .await
            }

            ResVariant::Redirect { location, target } => {
                response_writer.hx_redirect(&location, &target);
                Ok(())
            }

            ResVariant::Content { body, content_type } => {
                response_writer.set_header("Content-Type", &content_type);
                response_writer.write_body(&body).await
            }
        }
    }
}

impl SetHeader for Res {
    fn set_header(&mut self, key: &str, value: &str) -> &Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

impl HxHeaders for Res {}
impl HxHeaders for HttpResponseWriter {}

impl Elem {
    pub fn res(self) -> Res {
        Res::html(self)
    }
}

impl HttpResponseWriter {
    pub async fn css(&mut self, css: &[u8]) {
        self.set_header("Content-Type", "text/css");
        let _ = self.write_body(css).await;
    }
}
