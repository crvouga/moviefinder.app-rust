use super::html;

impl html::Elem {
    pub fn src_htmx_preload(self) -> Self {
        self.src("https://unpkg.com/htmx-ext-preload@2.0.1/preload.js")
    }

    pub fn hx_ext_preload(self) -> Self {
        self.hx_ext("preload")
    }

    pub fn hx_preload(self, value: &str) -> Self {
        self.attr("preload", value)
    }

    pub fn hx_preload_mouse_down(self) -> Self {
        self.hx_preload("mousedown")
    }
}
