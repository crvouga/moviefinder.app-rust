// https://htmx.org/docs/
// https://v1.htmx.org/extensions/loading-states/
use crate::core::html;
use serde::{Deserialize, Serialize};

use super::css;

impl html::Elem {
    pub fn hx_trigger(self, trigger: Trigger) -> Self {
        self.attr("hx-trigger", trigger.as_str())
    }

    pub fn hx_trigger_click(self) -> Self {
        self.hx_trigger(Trigger::Click)
    }

    pub fn hx_trigger_load(self) -> Self {
        self.hx_trigger(Trigger::Load)
    }

    pub fn hx_trigger_custom(self, value: &str) -> Self {
        self.hx_trigger(Trigger::Custom(value.to_string()))
    }

    pub fn hx_trigger_intersect(self) -> Self {
        self.hx_trigger(Trigger::Intersect)
    }

    pub fn hx_preload(self, preload: Preload) -> Self {
        self.attr("preload", preload.as_str()).clone()
    }

    pub fn hx_preload_mouse_down(self) -> Self {
        self.hx_preload(Preload::MouseDown)
    }

    pub fn hx_swap(self, swap: Swap) -> Self {
        self.attr("hx-swap", swap.as_str()).clone()
    }

    pub fn hx_swap_outer_html(self) -> Self {
        self.hx_swap(Swap::OuterHtml)
    }

    pub fn hx_swap_none(self) -> Self {
        self.hx_swap(Swap::None)
    }

    pub fn hx_swap_inner_html(self) -> Self {
        self.hx_swap(Swap::InnerHtml)
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

    pub fn hx_replace_url(self) -> Self {
        self.attr("hx-replace-url", "true").clone()
    }

    pub fn hx_target(self, css_selector: &str) -> Self {
        if css::selector::is_valid(css_selector) {
            self.attr("hx-target", css_selector).clone()
        } else {
            self
        }
    }

    pub fn hx_vals(self, values: &str) -> Self {
        self.attr("hx-vals", values).clone()
    }

    pub fn hx_ext(self, extensions: Vec<&str>) -> Self {
        self.attr("hx-ext", &extensions.join(" "))
    }

    pub fn hx_boost(self) -> Self {
        self.attr("hx-boost", "true")
    }

    pub fn hx_loading_aria_busy(self) -> Self {
        self.attr("data-loading-aria-busy", "")
    }

    pub fn hx_loading_disabled(self) -> Self {
        self.attr("data-loading-disable", "")
    }

    pub fn hx_loading_states(self) -> Self {
        self.attr("data-loading-states", "")
    }

    pub fn hx_on(self, event: &str, javascript: &str) -> Self {
        self.attr(&format!("hx-on:{}", event), javascript)
    }

    pub fn hx_abort(self, css_selector: &str) -> Self {
        if css::selector::is_valid(css_selector) {
            self.attr(
                "onclick",
                format!("htmx.trigger('{}', 'htmx:abort')", css_selector).as_str(),
            )
        } else {
            self
        }
    }
}

// https://htmx.org/attributes/hx-trigger/
pub enum Trigger {
    Load,
    Click,
    Intersect,
    Custom(String),
}

impl Trigger {
    pub fn as_str(&self) -> &str {
        match self {
            Trigger::Intersect => "intersect",
            Trigger::Load => "load",
            Trigger::Click => "click",
            Trigger::Custom(value) => value,
        }
    }
}

pub enum Swap {
    InnerHtml,
    OuterHtml,
    None,
}

impl Swap {
    pub fn as_str(&self) -> &str {
        match self {
            Swap::InnerHtml => "innerHTML",
            Swap::OuterHtml => "outerHTML",
            Swap::None => "none",
        }
    }
}

pub enum Preload {
    MouseDown,
}

impl Preload {
    pub fn as_str(&self) -> &str {
        match self {
            Preload::MouseDown => "mousedown",
        }
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
