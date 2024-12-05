#![allow(dead_code)]
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

impl Default for Elem {
    fn default() -> Self {
        Elem::Frag(vec![])
    }
}

impl Elem {
    pub fn map(self, f: impl FnOnce(Elem) -> Elem) -> Elem {
        f(self)
    }

    pub fn when(self, condition: bool, f: impl FnOnce(Elem) -> Elem) -> Elem {
        if condition {
            f(self)
        } else {
            self
        }
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

    pub fn a(self) -> Self {
        self.tag_name("a")
    }

    pub fn recursively_map_attrs(mut self, f: impl Fn(String, String) -> (String, String)) -> Self {
        let mut stack = vec![self.clone()];

        while let Some(elem) = stack.pop() {
            match elem {
                Elem::Tag {
                    tag_name,
                    attrs,
                    mut children,
                } => {
                    let mapped_attrs: HashMap<String, String> = attrs
                        .into_iter()
                        .map(|(name, value)| f(name, value))
                        .collect();

                    for child in children.drain(..) {
                        stack.push(child);
                    }

                    self = Elem::Tag {
                        tag_name,
                        attrs: mapped_attrs,
                        children,
                    };
                }
                Elem::Frag(mut children) => {
                    for child in children.drain(..) {
                        stack.push(child);
                    }

                    self = Elem::Frag(children);
                }
                _ => {
                    self = elem;
                }
            }
        }

        self
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

pub fn slot(name: &str) -> Elem {
    elem("slot").attr("name", name)
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

pub fn code() -> Elem {
    elem("code")
}

pub fn pre() -> Elem {
    elem("pre")
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

pub fn span() -> Elem {
    elem("span")
}
