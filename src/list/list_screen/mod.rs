use crate::{
    core::{
        html::{div, p, Elem},
        http::response_writer::ResponseWriter,
        pagination::Paginated,
        ui::top_bar::TopBar,
    },
    ctx::Ctx,
    list::list_item::ListItem,
};

use super::list::List;

pub mod route;

#[derive(Clone)]
pub struct ListScreen<T: List> {
    list: Option<T>,
    list_items: Option<Paginated<ListItem>>,
    back_url: Option<String>,
}

impl<T: List> Default for ListScreen<T> {
    fn default() -> Self {
        Self {
            list: None,
            back_url: None,
            list_items: None,
        }
    }
}

impl<T: List + 'static> ListScreen<T> {
    pub fn list(mut self, list: T) -> Self {
        self.list = Some(list);
        self
    }

    pub fn back_url(mut self, back_url: String) -> Self {
        self.back_url = Some(back_url);
        self
    }

    pub async fn list_items(mut self, list_items: Paginated<ListItem>) -> Self {
        self.list_items = Some(list_items);
        self
    }

    pub async fn respond_add_list_items(
        &self,
        _ctx: &Ctx,
        _list_items: Paginated<ListItem>,
        _w: &mut ResponseWriter,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    pub fn view(self) -> Elem {
        let list = self.list.clone();
        let name = list.clone().map(|l| l.name());
        let art = list.map(|l| l.view_art("size-32 rounded shrink-0"));

        div()
            .class("w-full h-full flex flex-col")
            .child(
                TopBar::default()
                    .back_url(self.back_url.unwrap_or("".to_owned()))
                    .title(&name.clone().unwrap_or_default())
                    .view(),
            )
            .child(
                div()
                    .class("w-full flex items-center justify-center pt-12 flex-col gap-6 px-6")
                    .child(art.unwrap_or_default())
                    .child(
                        p().class("w-full text-center text-3xl font-bold")
                            .child_text(&name.unwrap_or_default()),
                    ),
            )
            .child(div().id("list-items").class("w-full flex flex-col gap-4"))
    }
}

#[derive(Clone, Default)]
pub struct ViewModel<TList: List> {
    pub list: Option<TList>,
    pub list_items: Option<Paginated<ListItem>>,
    pub back_url: Option<String>,
}

impl<T: List> ViewModel<T> {
    pub fn view(self) -> Elem {
        div()
    }
}
