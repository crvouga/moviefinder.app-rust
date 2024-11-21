use crate::core::html::Elem;

const TAILWINDCSS_SCRIPT: &str = r#"
tailwind.config = {
    theme: {
        extend: {
            borderColor: {
                DEFAULT: '#3f3f46',
            },
        },
    },
}
"#;

impl Elem {
    pub fn js_tailwindcss_theme(self) -> Self {
        self.child_unsafe_text(TAILWINDCSS_SCRIPT)
    }
}
