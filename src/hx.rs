// https://htmx.org/docs/
use crate::html;

pub fn get(href: &str) -> html::Attr {
    html::attr("hx-get", href)
}

// https://htmx.org/attributes/hx-trigger/
pub enum Trigger {
    Load,
    MouseDown,
}

impl Trigger {
    pub fn to_str(&self) -> &str {
        match self {
            Trigger::Load => "load",
            Trigger::MouseDown => "mousedown",
        }
    }

    pub fn attr(&self) -> html::Attr {
        html::attr("hx-trigger", self.to_str())
    }
}

pub fn boost() -> html::Attr {
    return html::attr("hx-boost", "true");
}

pub fn target(selector: &str) -> html::Attr {
    html::attr("hx-target", selector)
}

pub enum Swap {
    InnerHtml,
    OuterHtml,
}

impl Swap {
    pub fn to_str(&self) -> &str {
        match self {
            Swap::InnerHtml => "innerHTML",
            Swap::OuterHtml => "outerHTML",
        }
    }

    pub fn attr(&self) -> html::Attr {
        html::attr("hx-swap", self.to_str())
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
    pub fn attr(&self) -> html::Attr {
        html::attr("preload", self.to_str())
    }
}
