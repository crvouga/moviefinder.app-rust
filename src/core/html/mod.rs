use std::collections::HashMap;

pub mod attr;
pub mod children;
mod escape;
pub mod render;

#[derive(Clone, Debug)]
pub enum Elem {
    Tag {
        tag_name: String,
        attrs: HashMap<String, String>,
        children: Vec<Elem>,
    },
    Frag(Vec<Elem>),
    Text(String),
}

impl Elem {
    pub fn map(self, f: impl FnOnce(Elem) -> Elem) -> Elem {
        f(self)
    }

    pub fn tag_name(mut self, tag_name_new: &str) -> Self {
        if let Elem::Tag {
            ref mut tag_name, ..
        } = self
        {
            *tag_name = tag_name_new.to_string();
        }

        self
    }

    pub fn button(self) -> Self {
        self.tag_name("button")
    }
}

pub fn unsafe_html(content: &str) -> Elem {
    Elem::Text(content.to_string())
}

pub fn frag() -> Elem {
    Elem::Frag(vec![])
}

pub fn elem(tag_name: &str) -> Elem {
    Elem::Tag {
        tag_name: tag_name.to_string(),
        attrs: HashMap::new(),
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

pub fn style() -> Elem {
    elem("style")
}

pub fn div() -> Elem {
    elem("div")
}

pub fn img() -> Elem {
    elem("img")
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
