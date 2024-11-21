use super::{css, html::Elem};

impl Elem {
    pub fn src_htmx_loading_states(self) -> Self {
        self.src("https://unpkg.com/htmx.org@1.9.12/dist/ext/loading-states.js")
    }

    pub fn css_htmx_loading_states(self) -> Self {
        self.child_unsafe_text(
            r#"
            [data-loading] {
                display: none;
            }
            "#,
        )
    }

    pub fn hx_ext_loading_states(self) -> Self {
        self.hx_ext("loading-states")
    }

    pub fn hx_loading_aria_busy(self) -> Self {
        self.attr("data-loading-aria-busy", "")
    }

    pub fn hx_loading_states(self) -> Self {
        self.attr("data-loading-states", "")
    }

    pub fn hx_loading_disabled(self) -> Self {
        self.attr("data-loading-disable", "")
    }

    pub fn hx_loading_path(self, path: &str) -> Self {
        self.attr("data-loading-path", path)
    }

    pub fn hx_loading_target(self, css_selector: &str) -> Self {
        if css::selector::is_valid(css_selector) {
            self.attr("data-loading-target", css_selector)
        } else {
            self
        }
    }
}
