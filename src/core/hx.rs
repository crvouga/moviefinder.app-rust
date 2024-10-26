// https://htmx.org/docs/
use crate::core::html;

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
    pub fn to_str(&self) -> &str {
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
        html::attr("hx-trigger", trigger.to_str())
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
    pub fn to_str(&self) -> &str {
        match self {
            Swap::InnerHtml => "innerHTML",
            Swap::OuterHtml => "outerHTML",
            Swap::None => "none",
        }
    }
}

impl From<Swap> for html::Attr {
    fn from(swap: Swap) -> Self {
        html::attr("hx-swap", swap.to_str())
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
    pub fn to_str(&self) -> &str {
        match self {
            Preload::MouseDown => "mousedown",
        }
    }
}

impl From<Preload> for html::Attr {
    fn from(preload: Preload) -> Self {
        html::attr("preload", preload.to_str())
    }
}
