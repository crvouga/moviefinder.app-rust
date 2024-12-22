use super::list_id::ListId;
use crate::core::html::{div, p, Elem};

pub trait List {
    fn id(&self) -> ListId;
    fn view_art(&self, class: &str) -> Elem;
    fn name(&self) -> String;
    fn details_url(&self) -> String;
}

pub fn view_lists<T: List>(lists: Option<Vec<T>>) -> Elem {
    div()
        .id("lists")
        .class("w-full flex flex-col")
        .children(match lists {
            None => (0..6).map(|_| view_list_loading()).collect(),
            Some(lists) => lists.into_iter().map(|list| view_list(list)).collect(),
        })
}

fn view_list_root() -> Elem {
    div().class("w-full flex items-center gap-4 px-4 py-2")
}

const ART_CLASS: &str = "size-16 aspect-square rounded";

fn view_list_loading() -> Elem {
    view_list_root()
        .class("select-none pointer-events-none animate-pulse")
        .child(div().class(ART_CLASS).class("bg-skeleton"))
        .child(
            div()
                .class("bg-skeleton rounded")
                .child(p().class("text-transparent").child_text("Loading")),
        )
}

fn view_list<T: List>(list: T) -> Elem {
    view_list_root()
        .button()
        .data_on(|e| e.click().push_then_sse(&list.details_url()))
        .class("active:opacity-50 cursor-pointer")
        .child(list.view_art(ART_CLASS))
        .child(p().child_text(&list.name()))
}
