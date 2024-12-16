use crate::core::{html::*, ui::icon};

#[derive(Default)]
pub struct TopBarRoot {}

impl TopBarRoot {
    pub fn view(&self) -> Elem {
        div().class("flex items-center justify-center shrink-0 w-full border-b h-bar font-bold text-lg text-center truncate")
    }
}

#[derive(Default)]
pub struct TopBar {
    back_url: Option<String>,
    title: Option<String>,
    cancel_url: Option<String>,
}

impl TopBar {
    pub fn back_url(mut self, value: String) -> Self {
        self.back_url = Some(value);
        self
    }

    pub fn title(mut self, value: &str) -> Self {
        self.title = Some(value.to_string());
        self
    }

    pub fn view(self) -> Elem {
        let back_button_elem = self
            .back_url
            .map_or(Empty::view(), |route| BackButton::new(route).view());

        let title_elem = self
            .title
            .map_or(div().class("flex-1 truncate"), |s| Title::view(&s));

        let cancel_button_elem = self
            .cancel_url
            .map_or(Empty::view(), |url| CancelButton::new(url).view());

        TopBarRoot::default()
            .view()
            .child(back_button_elem)
            .child(title_elem)
            .child(cancel_button_elem)
    }
}

#[derive(Default)]
pub struct BackButton {
    url: Option<String>,
}

impl BackButton {
    pub fn new(url: String) -> Self {
        Self {
            url: Some(url),
            ..Self::default()
        }
    }

    pub fn view(self) -> Elem {
        button()
            .class("size-16 flex items-center justify-center")
            .aria_label("go back")
            .type_("button")
            .map(|elem| match self.url {
                Some(url) => elem.data_on(|b| b.click().push_then_sse(&url)),
                None => elem,
            })
            .child(icon::solid::back_arrow("size-6"))
    }
}

#[derive(Default)]
pub struct CancelButton {
    url: Option<String>,
}

impl CancelButton {
    pub fn new(url: String) -> Self {
        Self {
            url: Some(url),
            ..Self::default()
        }
    }

    pub fn view(self) -> Elem {
        button()
            .class("size-16 flex items-center justify-center shrink-0")
            .class("disabled:opacity-80 disabled:cursor-not-allowed")
            .aria_label("cancel")
            .type_("button")
            .tab_index(0)
            .data_on(|b| {
                b.click()
                    .push_then_sse(&self.url.clone().unwrap_or("".to_string()))
            })
            .child(icon::solid::x_mark("size-8"))
    }
}

struct Title {}

impl Title {
    fn view(title: &str) -> Elem {
        div()
            .class("flex-1 text-center flex items-center justify-center h-full truncate max-w-full")
            .child(div().class("w-full truncate").child_text(title))
    }
}

struct Empty {}

impl Empty {
    fn view() -> Elem {
        div().class("size-16")
    }
}
