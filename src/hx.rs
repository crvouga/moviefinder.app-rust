// https://htmx.org/docs/
use crate::html;

pub fn get(href: &str) -> html::Attr {
    html::attr("hx-get", href)
}

pub enum Trigger {
    Load,
}

impl Trigger {
    pub fn to_str(&self) -> &str {
        match self {
            Trigger::Load => "load",
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
}

impl Swap {
    pub fn to_str(&self) -> &str {
        match self {
            Swap::InnerHtml => "innerHTML",
        }
    }

    pub fn attr(&self) -> html::Attr {
        html::attr("hx-swap", self.to_str())
    }
}

pub fn push_url(value: &str) -> html::Attr {
    html::attr("hx-push-url", value)
}
