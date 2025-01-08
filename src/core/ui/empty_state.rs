use crate::core::html::{div, frag, p, Html};

#[derive(Default)]
pub struct EmptyState {
    title: String,
    subtitle: String,
    icon: Option<Box<dyn Fn(String) -> Html + 'static>>,
}

impl EmptyState {
    pub fn title(mut self, label: impl Into<String>) -> Self {
        self.title = label.into();
        self
    }

    #[allow(dead_code)]
    pub fn icon(mut self, icon: impl Fn(String) -> Html + 'static) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }

    pub fn view(self) -> Html {
        let icon = self.icon.unwrap_or_else(|| Box::new(|_| frag()));
        div()
            .class("w-full flex flex-col items-center justify-center gap-4 py-20")
            .child(icon("size-24".to_owned()))
            .child(if self.title.len() > 0 {
                p().class("text-2xl font-bold text-muted")
                    .child_text(&self.title)
            } else {
                frag()
            })
            .child(if self.subtitle.len() > 0 {
                p().class("text-base text-muted").child_text(&self.subtitle)
            } else {
                frag()
            })
    }
}
