use super::{Attr, Elem};

impl Elem {
    pub fn attr(&self, name: &str, value: &str) -> Self {
        match self {
            Elem::Element {
                tag_name,
                attributes,
                children,
            } => {
                let mut new_attrs = attributes
                    .clone()
                    .into_iter()
                    .filter(|attr| attr.name != name)
                    .collect::<Vec<Attr>>();

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

    fn attrs(&self) -> Vec<Attr> {
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
        let existing_classes = self
            .attrs()
            .iter()
            .find(|attr| {
                attr.name.to_ascii_lowercase().trim() == "class".to_ascii_lowercase().trim()
            })
            .map(|attr| attr.value.clone())
            .unwrap_or("".to_string());

        let class_new = format!("{} {}", existing_classes, value);

        self.attr("class", class_new.trim())
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

    pub fn disabled(&self, value: bool) -> Elem {
        if value {
            self.attr("disabled", "true")
        } else {
            self.clone()
        }
    }
    pub fn aria_label(self, value: &str) -> Elem {
        self.attr("aria-label", value)
    }

    pub fn width(&self, value: &str) -> Elem {
        self.attr("width", value)
    }

    pub fn height(&self, value: &str) -> Elem {
        self.attr("height", value)
    }

    pub fn src(&self, value: &str) -> Elem {
        self.attr("src", value)
    }

    pub fn defer(&self) -> Elem {
        self.attr("defer", "true")
    }

    pub fn for_(self, value: &str) -> Elem {
        self.attr("for", value)
    }
}
