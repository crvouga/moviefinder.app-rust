use crate::core::html::Elem;

pub fn js_get(url: &str) -> String {
    format!("$get('{}')", url)
}

pub fn js_post(url: &str) -> String {
    format!("$post('{}', this)", url)
}

impl Elem {
    pub fn src_datastar(self) -> Self {
        self.src("https://cdn.jsdelivr.net/gh/starfederation/datastar/bundles/datastar.js")
    }

    pub fn data_model(self, value: &str) -> Self {
        self.attr("data-model", value)
    }

    pub fn data_store(self, value: &str) -> Self {
        self.attr("data-store", value)
    }

    pub fn data_on(self, event: &str, value: &str) -> Self {
        self.attr(&format!("data-on-{}", event), value)
    }

    pub fn data_on_then_post(self, event: &str, url: &str) -> Self {
        self.data_on(event, &js_post(url))
    }

    pub fn data_on_click(self, value: &str) -> Self {
        self.data_on("click", value)
    }

    pub fn data_on_click_get(self, url: &str) -> Self {
        self.data_on_click(&js_get(url))
    }

    pub fn data_on_click_post(self, url: &str) -> Self {
        self.data_on_click(&format!("$post('{}', this)", url))
    }
}
