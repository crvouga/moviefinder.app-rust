use crate::{
    core::{html::*, ui::icon},
    route::Route,
};

#[derive(Default)]
pub struct TopBar {
    back_route: Option<Route>,
    title: Option<String>,
    cancel_route: Option<Route>,
}

impl TopBar {
    pub fn back_button(mut self, back_route: Route) -> Self {
        self.back_route = Some(back_route);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn view(self) -> Elem {
        let back_button_elem = self.back_route.map_or(Empty::view(), BackButton::view);

        let title_elem = self
            .title
            .map_or(div().class("flex-1"), |s| Title::view(&s));

        let cancel_button_elem = self.cancel_route.map_or(Empty::view(), CancelButton::view);

        div()
        .class("flex items-center justify-center w-full border-b h-16 font-bold text-lg text-center truncate")
        .child(back_button_elem)
        .child(title_elem)
        .child(cancel_button_elem)
    }
}

pub struct BackButton {}

impl BackButton {
    pub fn view(back_route: Route) -> Elem {
        button()
            .class("size-16 flex items-center justify-center")
            .aria_label("go back")
            .hx_push_screen(back_route)
            .child(icon::back_arrow("size-8"))
    }
}

pub struct CancelButton {}

impl CancelButton {
    pub fn view(cancel_route: Route) -> Elem {
        button()
            .class("size-16 flex items-center justify-center")
            .hx_push_screen(cancel_route)
            .aria_label("cancel")
            .child(icon::x_mark("size-8"))
    }
}

struct Title {}

impl Title {
    fn view(title: &str) -> Elem {
        div()
            .class("flex-1 text-center flex items-center justify-center h-full truncate")
            .child_text(title)
    }
}

struct Empty {}

impl Empty {
    fn view() -> Elem {
        div().class("size-16")
    }
}
