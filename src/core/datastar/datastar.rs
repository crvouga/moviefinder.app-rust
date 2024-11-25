use crate::core::html::Elem;

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
}
