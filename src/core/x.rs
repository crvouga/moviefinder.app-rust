// https://alpinejs.dev/
use super::html::Elem;

impl Elem {
    pub fn x_data(self, javascript: &str) -> Self {
        self.attr("x-data", javascript)
    }

    pub fn x_show(self, javascript: &str) -> Self {
        self.attr("x-show", javascript)
    }

    pub fn x_on(self, event: &str, javascript: &str) -> Self {
        self.attr(&format!("x-on:{}", event), javascript)
    }

    pub fn x_model(self, javascript: &str) -> Self {
        self.attr("x-model", javascript)
    }

    pub fn x_ref(self, javascript: &str) -> Self {
        self.attr("x-ref", javascript)
    }
}
