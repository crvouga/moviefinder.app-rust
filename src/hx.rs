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
