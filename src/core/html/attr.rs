use super::{Attr, Elem};

impl Elem {
    pub fn attr(&self, name: &str, value: &str) -> Self {
        match self {
            Elem::Element {
                tag_name,
                attributes,
                children,
            } => {
                let mut new_attrs = attributes.clone();
                new_attrs.push(Attr {
                    name: name.to_string(),
                    value: value.to_string(),
                });
                Elem::Element {
                    tag_name: tag_name.clone(),
                    attributes: new_attrs,
                    children: children.clone(),
                }
            }
            _ => self.clone(),
        }
    }

    fn get_attrs(&self) -> Vec<Attr> {
        match self {
            Elem::Element {
                tag_name: _,
                attributes,
                children: _,
            } => attributes.clone(),
            _ => vec![],
        }
    }

    pub fn class(&self, value: &str) -> Elem {
        let class_str = self
            .get_attrs()
            .iter()
            .fold(value.to_string(), |acc, attr| {
                if attr.name == "class" {
                    format!("{} {}", value, acc)
                } else {
                    acc
                }
            });

        self.attr("class", &class_str)
    }

    pub fn class_list(&self, class_names: &[&str]) -> Elem {
        self.class(&class_names.join(" "))
    }

    pub fn type_(&self, type_: &str) -> Elem {
        self.attr("type", type_)
    }

    pub fn href(&self, value: &str) -> Elem {
        self.attr("href", value)
    }

    pub fn lang(&self, lang: &str) -> Elem {
        self.attr("lang", lang)
    }

    pub fn id(&self, id: &str) -> Elem {
        self.attr("id", id)
    }

    pub fn name(&self, name: &str) -> Elem {
        self.attr("name", name)
    }

    pub fn rel(&self, value: &str) -> Elem {
        self.attr("rel", value)
    }

    pub fn content(&self, value: &str) -> Elem {
        self.attr("content", value)
    }

    pub fn charset(&self, value: &str) -> Elem {
        self.attr("charset", value)
    }

    pub fn value(&self, value: &str) -> Elem {
        self.attr("value", value)
    }

    pub fn checked(&self, checked: bool) -> Elem {
        if checked {
            self.attr("checked", "true")
        } else {
            self.clone()
        }
    }

    pub fn disabled(&self, disabled: bool) -> Elem {
        if disabled {
            self.attr("disabled", "true")
        } else {
            self.clone()
        }
    }
    pub fn aria_label(self, label: &str) -> Elem {
        self.attr("aria-label", label)
    }

    pub fn width(&self, width: &str) -> Elem {
        self.attr("width", width)
    }

    pub fn height(&self, height: &str) -> Elem {
        self.attr("height", height)
    }

    pub fn src(&self, src: &str) -> Elem {
        self.attr("src", src)
    }

    pub fn defer(&self) -> Elem {
        self.attr("defer", "true")
    }
}
