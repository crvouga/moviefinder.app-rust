use crate::core::html::Elem;
use crate::core::http::response::HttpResponse;

#[derive(Debug, Default)]
pub struct Res {
    pub variant: ResVariant,
    pub modifiers: Vec<ResModifier>,
}

#[derive(Debug)]
pub enum ResModifier {
    HxPushUrl(String),
    HxReplaceUrl(String),
    Cache,
}

#[derive(Debug, Default)]
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

    pub fn map_html(mut self, f: impl FnOnce(Elem) -> Elem) -> Self {
        self.variant = match self.variant {
            ResVariant::Html(body) => ResVariant::Html(f(body)),
            other => other,
        };
        self
    }

    pub fn hx_push_url(mut self, url: &str) -> Self {
        self.modifiers.push(ResModifier::HxPushUrl(url.to_owned()));
        self
    }

    pub fn hx_replace_url(mut self, url: &str) -> Self {
        self.modifiers
            .push(ResModifier::HxReplaceUrl(url.to_owned()));

        self
    }

    pub fn cache(mut self) -> Self {
        self.modifiers.push(ResModifier::Cache);
        self
    }

    pub fn no_cache(mut self) -> Self {
        self.modifiers.retain(|action| match action {
            ResModifier::Cache => false,
            _ => true,
        });
        self
    }
}

impl From<Elem> for Res {
    fn from(elem: Elem) -> Self {
        Res::html(elem)
    }
}

impl From<Res> for HttpResponse {
    fn from(res: Res) -> Self {
        res.modifiers.iter().fold(
            res.variant.into(),
            |mut http_response, modifier| match modifier {
                ResModifier::HxPushUrl(url) => http_response.hx_push_url(&url).to_owned(),

                ResModifier::HxReplaceUrl(url) => http_response.hx_replace_url(&url).to_owned(),

                ResModifier::Cache => http_response
                    .header("Cache-Control", "public, max-age=31536000, immutable")
                    .header("Access-Control-Expose-Headers", "Cache-Control"),
            },
        )
    }
}

impl From<ResVariant> for HttpResponse {
    fn from(res_variant: ResVariant) -> Self {
        match res_variant {
            ResVariant::Empty => HttpResponse::new(204),

            ResVariant::Html(body) => HttpResponse::new(200).body(body.render().into_bytes()),

            ResVariant::Redirect { location, target } => HttpResponse::new(302)
                .hx_redirect(&location, &target)
                .to_owned(),

            ResVariant::Content { body, content_type } => HttpResponse::new(200)
                .body(body)
                .header("Content-Type", &content_type),
        }
    }
}
