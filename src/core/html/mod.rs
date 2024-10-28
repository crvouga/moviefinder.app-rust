pub mod attr;
pub mod children;
pub mod render;

#[derive(Clone, Debug)]
pub struct Attr {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum Elem {
    Element {
        tag_name: String,
        attributes: Vec<Attr>,
        children: Vec<Elem>,
    },
    Fragment(Vec<Elem>),
    Safe(String),
    Unsafe(String),
}

pub fn unsafe_html(content: &str) -> Elem {
    Elem::Unsafe(content.to_string())
}

pub fn frag() -> Elem {
    Elem::Fragment(vec![])
}

pub fn elem(tag_name: &str) -> Elem {
    Elem::Element {
        tag_name: tag_name.to_string(),
        attributes: vec![],
        children: vec![],
    }
}

pub fn meta() -> Elem {
    elem("meta")
}

pub fn title() -> Elem {
    elem("title")
}

pub fn link() -> Elem {
    elem("link")
}

pub fn script() -> Elem {
    elem("script")
}

pub fn div() -> Elem {
    elem("div")
}

pub fn form() -> Elem {
    elem("form")
}

pub fn p() -> Elem {
    elem("p")
}

pub fn button() -> Elem {
    elem("button")
}

pub fn html() -> Elem {
    elem("html")
}

pub fn head() -> Elem {
    elem("head")
}

pub fn body() -> Elem {
    elem("body")
}

pub fn a() -> Elem {
    elem("a")
}

pub fn input() -> Elem {
    elem("input")
}

pub fn label() -> Elem {
    elem("label")
}
