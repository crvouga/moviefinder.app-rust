// https://htmx.org/docs/
// https://v1.htmx.org/extensions/loading-states/
use crate::core::{
    css,
    html::Elem,
    http::{response::Response, set_header::SetHeader},
};
use serde::{Deserialize, Serialize};

impl Elem {
    pub fn src_htmx(self) -> Self {
        self.src("https://unpkg.com/htmx.org@2.0.1")
    }

    pub fn hx_trigger(mut self, value: &str) -> Self {
        if let Elem::Tag {
            attrs: ref mut attributes,
            ..
        } = self
        {
            let existing = attributes
                .get("hx-trigger")
                .map_or("", |attr| attr.as_str());

            let new = if existing.is_empty() {
                value.trim().to_string()
            } else {
                format!("{}, {}", existing, value).trim().to_string()
            };

            attributes.insert("hx-trigger".to_string(), new);
        }

        self
    }

    pub fn hx_trigger_click(self) -> Self {
        self.hx_trigger("click")
    }

    pub fn hx_trigger_load(self) -> Self {
        self.hx_trigger("load")
    }

    pub fn hx_trigger_focus(self) -> Self {
        self.hx_trigger("focus")
    }

    pub fn hx_trigger_intersect(self) -> Self {
        self.hx_trigger("intersect")
    }

    pub fn hx_swap(self, value: &str) -> Self {
        self.attr("hx-swap", value)
    }

    pub fn hx_swap_outer_html(self) -> Self {
        self.hx_swap("outerHTML")
    }

    pub fn hx_swap_none(self) -> Self {
        self.hx_swap("none")
    }

    pub fn hx_swap_inner_html(self) -> Self {
        self.hx_swap("innerHTML")
    }

    pub fn hx_get(self, href: &str) -> Self {
        self.attr("hx-get", href).clone()
    }

    pub fn hx_post(self, href: &str) -> Self {
        self.attr("hx-post", href).clone()
    }

    pub fn hx_push_url(self) -> Self {
        self.attr("hx-push-url", "true").clone()
    }

    pub fn hx_target(self, css_selector: &str) -> Self {
        if css::selector::is_valid(css_selector) {
            self.attr("hx-target", css_selector).clone()
        } else {
            self
        }
    }

    pub fn hx_target_this(self) -> Self {
        self.hx_target("this")
    }

    pub fn hx_vals(self, values: &str) -> Self {
        self.attr_unsafe("hx-vals", values).clone()
    }

    pub fn hx_include(self, value: &str) -> Self {
        self.attr("hx-include", value).clone()
    }

    pub fn hx_include_this(self) -> Self {
        self.hx_include("this")
    }

    /// https://htmx.org/attributes/hx-ext/
    pub fn hx_ext(mut self, value: &str) -> Self {
        if let Elem::Tag {
            attrs: ref mut attributes,
            ..
        } = self
        {
            let existing = attributes.get("hx-ext").map_or("", |attr| attr.as_str());

            let new = if existing.is_empty() {
                value.trim().to_string()
            } else {
                format!("{}, {}", existing, value).trim().to_string()
            };

            attributes.insert("hx-ext".to_string(), new);
        }

        self
    }

    pub fn hx_boost(self) -> Self {
        self.attr("hx-boost", "true")
    }

    pub fn hx_preserve(self) -> Self {
        self.attr("hx-preserve", "true")
    }

    pub fn hx_abort(self, css_selector: &str) -> Self {
        if css::selector::is_valid(css_selector) {
            self.attr_unsafe(
                "onclick",
                format!("htmx.trigger('{}', 'htmx:abort')", css_selector).as_str(),
            )
        } else {
            self
        }
    }

    pub fn hx_on(self, event: &str, value: &str) -> Self {
        self.attr_unsafe(format!("hx-on:{}", event).as_str(), value)
    }
}

// https://htmx.org/headers/hx-location/

#[derive(Serialize, Deserialize, Debug)]
pub struct HxLocation {
    path: String,
    target: String,
}

impl HxLocation {
    pub fn new(path: String, target: String) -> Self {
        Self { path, target }
    }
}

pub trait HxHeaders: SetHeader + Sized {
    fn hx_push_url(&mut self, url: &str) -> &Self {
        self.set_header("HX-Push-Url", url);
        self.set_header("Access-Control-Expose-Headers", "HX-Push-Url");
        self
    }

    fn hx_replace_url(&mut self, url: &str) -> &Self {
        self.set_header("HX-Replace-Url", &ensure_leading_slash(url));
        self.set_header("Access-Control-Expose-Headers", "HX-Replace-Url");
        self
    }

    fn hx_redirect(&mut self, location: &str, target: &str) -> &Self {
        let hx_location = HxLocation::new(location.to_string(), target.to_string());
        let location_str =
            serde_json::to_string(&hx_location).unwrap_or_else(|_| location.to_string());
        self.set_header("HX-Location", &location_str);
        self.set_header("Access-Control-Expose-Headers", "HX-Location");
        self
    }

    fn hx_retarget(&mut self, target: &str) -> &Self {
        self.set_header("HX-Retarget", target);
        self.set_header("Access-Control-Expose-Headers", "HX-Retarget");
        self
    }

    fn hx_retarget_outer_html(&mut self) -> &Self {
        self.hx_retarget("outerHTML");
        self
    }

    fn hx_reswap(&mut self, target: &str) -> &Self {
        self.set_header("HX-Reswap", target);
        self.set_header("Access-Control-Expose-Headers", "HX-Reswap");
        self
    }
}

impl HxHeaders for Response {}

fn ensure_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{}", path)
    }
}
