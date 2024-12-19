use crate::core::{
    html::{children::text_unsafe, frag, img, span, Elem},
    js::Js,
};

#[derive(Default)]
pub struct Avatar {
    data_attributes_src: String,
    class: String,
    on_click: Option<String>,
}

impl Avatar {
    pub fn data_attributes_src(mut self, data_attributes_src: &str) -> Self {
        self.data_attributes_src = data_attributes_src.to_string();
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.class = class.to_string();
        self
    }

    pub fn view(self) -> Elem {
        let class = format!(
            "inline-block aspect-square flex-shrink-0 overflow-hidden rounded-full bg-white/40 {}",
            self.class,
        );

        frag()
        .child(
            span()
                .class(&class)
                .data_show(&Js::not(&Js::is_str(&self.data_attributes_src)))
                .data_on(|b| {
                    if let Some(on_click) = &self.on_click {
                        b.press_down().js(on_click)
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
        ).child(
            img()
                .alt("avatar")
                .class(&class)
                .data_attributes("src", &self.data_attributes_src)
                .data_show(&Js::is_str(&self.data_attributes_src))
                .data_on(|b| {
                    if let Some(on_click) = &self.on_click {
                        b.press_down().js(on_click)
                    } else {
                        b
                    }
                })
            )
    }
}
