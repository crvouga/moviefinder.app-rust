use crate::core::html::{children::text_unsafe, img, span, Elem};

#[derive(Default)]
pub struct Avatar {
    src: Option<String>,
    alt: Option<String>,
    class_name: Option<String>,
    on_click: Option<String>,
}

impl Avatar {
    pub fn src(mut self, src: &str) -> Self {
        self.src = Some(src.to_string());
        self
    }

    pub fn view(self) -> Elem {
        let class_name = format!(
            "inline-block aspect-square flex-shrink-0 overflow-hidden rounded-full bg-white/40 {}",
            self.class_name.unwrap_or_default()
        );

        let binding = self.src.unwrap_or_default();
        let src = binding.trim();

        if src.is_empty() {
            span()
                .class(&class_name)
                .data_on(|b| {
                    if let Some(on_click) = &self.on_click {
                        b.click().js(on_click)
                    } else {
                        b
                    }
                })
                .child(
                    text_unsafe(
                        r#"
                        <svg class='h-full w-full text-secondary' fill='currentColor' viewBox='0 0 24 24'>
                            <path d='M24 20.993V24H0v-2.996A14.977 14.977 0 0112.004 15c4.904 0 9.26 2.354 11.996 5.993zM16.002 8.999a4 4 0 11-8 0 4 4 0 018 0z' />
                        </svg>
                        "#,
                    )
                )
        } else {
            img()
                .src(&src)
                .alt(self.alt.as_deref().unwrap_or("user avatar"))
                .class(&class_name)
                .data_on(|b| {
                    if let Some(on_click) = &self.on_click {
                        b.click().js(on_click)
                    } else {
                        b
                    }
                })
        }
    }
}
