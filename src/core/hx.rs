// https://htmx.org/docs/
use crate::core::html;
use serde::{Deserialize, Serialize};

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

    pub fn hx_target(self, selector: &str) -> Self {
        self.attr("hx-target", selector).clone()
    }

    pub fn hx_vals(self, values: &str) -> Self {
        self.attr("hx-vals", values).clone()
    }
}

pub fn get(href: &str) -> html::Attr {
    html::attr("hx-get", href)
}

pub fn post(href: &str) -> html::Attr {
    html::attr("hx-post", href)
}

pub fn vals(values: &str) -> html::Attr {
    html::attr("hx-vals", values)
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

impl From<Trigger> for html::Attr {
    fn from(trigger: Trigger) -> Self {
        html::attr("hx-trigger", trigger.as_str())
    }
}

pub fn boost() -> html::Attr {
    html::attr("hx-boost", "true")
}

pub fn target(selector: &str) -> html::Attr {
    html::attr("hx-target", selector)
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

impl From<Swap> for html::Attr {
    fn from(swap: Swap) -> Self {
        html::attr("hx-swap", swap.as_str())
    }
}

pub fn push_url(value: &str) -> html::Attr {
    html::attr("hx-push-url", value)
}

pub fn ext(value: &str) -> html::Attr {
    html::attr("hx-ext", value)
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

impl From<Preload> for html::Attr {
    fn from(preload: Preload) -> Self {
        html::attr("preload", preload.as_str())
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
