use crate::core::html::Elem;

impl Elem {
    pub fn js_tailwindcss_theme(self) -> Self {
        self.child_unsafe_text(
            r#"
            tailwind.config = {
                theme: {
                    extend: {
                        borderColor: {
                            DEFAULT: '#3f3f46',
                        },
                    },
                },
            }
            "#,
        )
    }
}
