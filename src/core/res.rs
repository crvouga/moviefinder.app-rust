use super::{htmx::hx::HxLocation, http::response::HttpResponse};
use crate::core::html::Elem;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Res {
    pub variant: ResVariant,
    pub actions: Vec<ResAction>,
}

#[derive(Debug)]
pub enum ResAction {
    HxPushUrl(String),
    HxReplaceUrl(String),
    Cache,
}

#[derive(Debug)]
pub enum ResVariant {
    Html(Elem),
    Redirect { location: String, target: String },
    RedirectWindow(String),
    Text(String),
    Css(String),
    Image(Vec<u8>),
    Empty,
}

impl Res {
    pub fn html(body: Elem) -> Self {
        Self {
            variant: ResVariant::Html(body),
            ..Res::default()
        }
    }

    pub fn empty() -> Self {
        Self {
            variant: ResVariant::Empty,
            ..Res::default()
        }
    }

    pub fn text(text: &str) -> Self {
        Self {
            variant: ResVariant::Text(text.to_owned()),
            ..Res::default()
        }
    }

    pub fn image(image: Vec<u8>) -> Self {
        Self {
            variant: ResVariant::Image(image),
            ..Res::default()
        }
    }

    pub fn css(css: &str) -> Self {
        Self {
            variant: ResVariant::Css(css.to_owned()),
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

    pub fn redirect_window(location: String) -> Self {
        Self {
            variant: ResVariant::RedirectWindow(location),
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

    pub fn hx_push_url(mut self, url: &str) -> Self {
        self.actions.push(ResAction::HxPushUrl(url.to_owned()));
        self
    }

    pub fn hx_replace_url(mut self, url: &str) -> Self {
        self.actions.push(ResAction::HxReplaceUrl(url.to_owned()));

        self
    }

    pub fn cache(mut self) -> Self {
        self.actions.push(ResAction::Cache);
        self
    }

    pub fn no_cache(mut self) -> Self {
        self.actions.retain(|action| match action {
            ResAction::Cache => false,
            _ => true,
        });
        self
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
                ResAction::HxPushUrl(url) => {
                    http_response
                        .headers
                        .insert("HX-Push-Url".to_string(), ensure_leading_slash(&url));
                    http_response.headers.insert(
                        "Access-Control-Expose-Headers".to_string(),
                        "HX-Push-Url".to_string(),
                    );
                }

                ResAction::HxReplaceUrl(url) => {
                    http_response
                        .headers
                        .insert("HX-Replace-Url".to_string(), ensure_leading_slash(&url));
                    http_response.headers.insert(
                        "Access-Control-Expose-Headers".to_string(),
                        "HX-Replace-Url".to_string(),
                    );
                }

                ResAction::Cache => {
                    http_response.headers.insert(
                        "Cache-Control".to_string(),
                        "public, max-age=31536000, immutable".to_string(),
                    );
                    http_response.headers.insert(
                        "Access-Control-Expose-Headers".to_string(),
                        "Cache-Control".to_string(),
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
            ResVariant::Html(body) => {
                HttpResponse::new(200, body.render().into_bytes(), HashMap::new())
            }
            ResVariant::Redirect { location, target } => {
                let mut headers = HashMap::new();
                headers.insert(
                    "HX-Location".to_string(),
                    serde_json::to_string(&HxLocation::new(location.clone(), target.clone()))
                        .unwrap_or(location.clone()),
                );
                headers.insert(
                    "Access-Control-Expose-Headers".to_string(),
                    "HX-Location".to_string(),
                );
                HttpResponse::new(302, vec![], headers)
            }
            ResVariant::RedirectWindow(location) => {
                let mut headers = HashMap::new();
                headers.insert("Location".to_string(), location.clone());
                HttpResponse::new(302, vec![], headers)
            }
            ResVariant::Text(text) => HttpResponse::new(200, text.into_bytes(), HashMap::new()),
            ResVariant::Css(css) => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "text/css".to_string());
                let res = HttpResponse::new(200, css.into_bytes(), headers);
                res
            }
            ResVariant::Image(image) => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "image/jpeg".to_string());
                HttpResponse::new(200, image, headers)
            }
            ResVariant::Empty => HttpResponse::new(204, vec![], HashMap::new()),
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
