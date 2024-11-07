// https://alpinejs.dev/
use super::html::Elem;

impl Elem {
    pub fn x_data(self, javascript: &str) -> Self {
        self.attr("x-data", javascript)
    }

    pub fn x_show(self, javascript: &str) -> Self {
        self.attr("x-show", javascript)
    }
}
