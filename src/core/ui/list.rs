use crate::core::html::{button, div, frag, p, Html};

#[derive(Default)]
pub struct ViewList;

impl ViewList {
    pub fn view(self) -> Html {
        div().class("w-full flex flex-col")
    }
}

#[derive(Default)]
pub struct ViewListItem {
    art: Option<Box<dyn FnOnce(String) -> Html>>,
    title: String,
    #[allow(dead_code)]
    subtitle: String,
    skeleton: bool,
}

impl ViewListItem {
    pub fn art(mut self, art: impl FnOnce(String) -> Html + 'static) -> Self {
        self.art = Some(Box::new(art));
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[allow(dead_code)]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn skeleton(mut self, skeleton: bool) -> Self {
        self.skeleton = skeleton;
        self
    }

    pub fn view(self) -> Html {
        const ART_CLASS: &str = "size-16 aspect-square rounded";
        let art = self.art.unwrap_or_else(|| Box::new(|_| frag()));
        button()
            .class("w-full flex items-center gap-4 px-4 py-2")
            .map(|e| {
                if self.skeleton {
                    e.class("select-none pointer-events-none animate-pulse")
                        .child(div().class(ART_CLASS).class("bg-skeleton"))
                        .child(
                            div().class("bg-skeleton rounded").child(
                                p().class("text-transparent").child_text(
                                    if self.title.is_empty() {
                                        "Loading"
                                    } else {
                                        &self.title
                                    },
                                ),
                            ),
                        )
                } else {
                    e.class("active:opacity-active cursor-pointer")
                        .child(art(ART_CLASS.to_owned()))
                        .child(p().class("text-left w-full").child_text(&self.title))
                }
            })
    }
}
