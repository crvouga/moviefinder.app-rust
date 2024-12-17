#![allow(dead_code)]
use super::{escape::escape, Elem};

impl Elem {
    pub fn attr_unsafe(mut self, name: &str, value: &str) -> Self {
        if let Elem::Tag { ref mut attrs, .. } = self {
            attrs.insert(name.to_string(), value.to_string());
        }
        self
    }

    pub fn attr(self, name: &str, value: &str) -> Self {
        self.attr_unsafe(name, &escape(value))
    }

    pub fn class(mut self, value: &str) -> Self {
        if let Elem::Tag { ref mut attrs, .. } = self {
            let class_existing = attrs.get("class").map_or("", |attr| attr.as_str());

            let class_new = if class_existing.is_empty() {
                value.trim().to_string()
            } else {
                format!("{} {}", class_existing, value).trim().to_string()
            };

            attrs.insert("class".to_string(), class_new);
        }

        self
    }

    pub fn style(self, value: &str) -> Self {
        self.attr("style", value)
    }

    pub fn class_list(self, class_names: &[&str]) -> Self {
        self.class(&class_names.join(" "))
    }

    pub fn type_(self, type_: &str) -> Self {
        self.attr("type", type_)
    }

    pub fn type_module(self) -> Self {
        self.type_("module")
    }

    pub fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    pub fn tab_index(self, value: i32) -> Self {
        self.attr("tabindex", &value.to_string())
    }

    pub fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    pub fn crossorigin(self) -> Self {
        self.attr("crossorigin", "")
    }

    pub fn lang(self, lang: &str) -> Self {
        self.attr("lang", lang)
    }

    pub fn id(self, id: &str) -> Self {
        self.attr("id", id)
    }

    pub fn namespace_children_ids(self, namepsace: &str) -> Self {
        self.recursively_map_attrs(|key, value| match key.to_lowercase().as_str() {
            "id" => (key.to_string(), format!("{}_{}", namepsace, value)),
            _ => (key.to_string(), value.to_string()),
        })
    }

    pub fn name(self, name: &str) -> Self {
        self.attr("name", name)
    }

    pub fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    pub fn content(self, value: &str) -> Self {
        self.attr("content", value)
    }

    pub fn charset(self, value: &str) -> Self {
        self.attr("charset", value)
    }

    pub fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }

    pub fn on_click(self, value: &str) -> Self {
        self.attr_unsafe("onclick", value)
    }

    pub fn on_load(self, value: &str) -> Self {
        self.attr_unsafe("onload", value)
    }

    pub fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }

    pub fn media_print(self) -> Self {
        self.media("print")
    }

    pub fn checked(self, checked: bool) -> Self {
        if checked {
            self.attr("checked", "true")
        } else {
            self
        }
    }

    pub fn disabled(self, value: bool) -> Self {
        if value {
            self.attr("disabled", "true")
        } else {
            self
        }
    }

    pub fn hidden(self, value: bool) -> Self {
        if value {
            self.attr("hidden", "true")
        } else {
            self
        }
    }

    pub fn selected(self, value: bool) -> Self {
        if value {
            self.attr("selected", "true")
        } else {
            self
        }
    }

    pub fn aria_label(self, value: &str) -> Self {
        self.attr("aria-label", value)
    }

    pub fn role(self, value: &str) -> Self {
        self.attr("role", value)
    }

    pub fn role_button(self) -> Self {
        self.role("button")
    }

    pub fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    pub fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    pub fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    pub fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    pub fn defer(self) -> Self {
        self.attr("defer", "true")
    }

    pub fn for_(self, value: &str) -> Self {
        self.attr("for", value)
    }

    pub fn placeholder(self, value: &str) -> Self {
        self.attr("placeholder", value)
    }
}
