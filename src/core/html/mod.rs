#![allow(dead_code)]
use std::collections::HashMap;

pub mod attr;
pub mod children;
mod escape;
pub mod render;

#[derive(Clone, Debug)]
pub enum Html {
    Tag {
        tag_name: String,
        attrs: HashMap<String, String>,
        children: Vec<Html>,
    },
    Frag(Vec<Html>),
    Text(String),
}

impl Default for Html {
    fn default() -> Self {
        Html::Frag(vec![])
    }
}

impl Html {
    pub fn map(self, f: impl FnOnce(Html) -> Html) -> Html {
        f(self)
    }

    pub fn when(self, condition: bool, f: impl FnOnce(Html) -> Html) -> Html {
        if condition {
            f(self)
        } else {
            self
        }
    }

    pub fn tag_name(mut self, tag_name_new: &str) -> Self {
        if let Html::Tag {
            ref mut tag_name, ..
        } = self
        {
            *tag_name = tag_name_new.to_string();
        }

        self
    }

    pub fn remove(self, should_remove: bool) -> Self {
        if should_remove {
            Self::Frag(vec![])
        } else {
            self
        }
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
                Html::Tag {
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

                    self = Html::Tag {
                        tag_name,
                        attrs: mapped_attrs,
                        children,
                    };
                }
                Html::Frag(mut children) => {
                    for child in children.drain(..) {
                        stack.push(child);
                    }

                    self = Html::Frag(children);
                }
                _ => {
                    self = elem;
                }
            }
        }

        self
    }
}

pub fn unsafe_text(content: &str) -> Html {
    Html::Text(content.to_string())
}

pub fn frag() -> Html {
    Html::Frag(vec![])
}

pub fn elem(tag_name: &str) -> Html {
    Html::Tag {
        tag_name: tag_name.to_string(),
        attrs: HashMap::new(),
        children: vec![],
    }
}

pub fn meta() -> Html {
    elem("meta")
}

pub fn title() -> Html {
    elem("title")
}

pub fn slot(name: &str) -> Html {
    elem("slot").attr("name", name)
}

pub fn link() -> Html {
    elem("link")
}

pub fn script() -> Html {
    elem("script")
}

pub fn style() -> Html {
    elem("style")
}

pub fn div() -> Html {
    elem("div")
}

pub fn main() -> Html {
    elem("main")
}

pub fn code() -> Html {
    elem("code")
}

pub fn pre() -> Html {
    elem("pre")
}

pub fn img() -> Html {
    elem("img")
}

pub fn form() -> Html {
    elem("form")
}

pub fn p() -> Html {
    elem("p")
}

pub fn button() -> Html {
    elem("button")
}

pub fn html() -> Html {
    elem("html")
}

pub fn head() -> Html {
    elem("head")
}

pub fn body() -> Html {
    elem("body")
}

pub fn a() -> Html {
    elem("a")
}

pub fn input() -> Html {
    elem("input")
}

pub fn label() -> Html {
    elem("label")
}

pub fn span() -> Html {
    elem("span")
}

pub fn select() -> Html {
    elem("select")
}

pub fn option() -> Html {
    elem("option")
}

pub fn fieldset() -> Html {
    elem("fieldset")
}

pub fn legend() -> Html {
    elem("legend")
}
