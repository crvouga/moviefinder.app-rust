use super::List;
use crate::core::{
    html::{div, p, Elem},
    ui::top_bar::TopBar,
};

#[derive(Clone)]
pub struct ListScreen<T: List + 'static> {
    list: T,
    back_url: Option<String>,
}

impl<T: List + 'static> ListScreen<T> {
    pub fn new(list: T) -> Self {
        Self {
            list,
            back_url: None,
        }
    }

    pub fn back_url(mut self, back_url: String) -> Self {
        self.back_url = Some(back_url);
        self
    }

    pub fn view(self) -> Elem {
        div()
            .class("w-full h-full flex flex-col")
            .child(
                TopBar::default()
                    .back_url(self.back_url.unwrap_or("".to_owned()))
                    .title(&self.list.name())
                    .view(),
            )
            .child(
                div()
                    .class("w-full flex items-center justify-center pt-12 flex-col gap-6 px-6")
                    .child(self.list.view_art("size-32 rounded shrink-0"))
                    .child(
                        p().class("w-full text-center text-3xl font-bold")
                            .child_text(&self.list.name()),
                    ),
            )
    }
}
