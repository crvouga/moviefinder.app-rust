use super::list::List;
use crate::core::{html::Html, ui};
use std::sync::Arc;

#[derive(Clone)]
pub struct ListSection<T: List + 'static> {
    lists: Option<Vec<T>>,
}

impl<T: List + 'static> Default for ListSection<T> {
    fn default() -> Self {
        Self { lists: None }
    }
}

impl<T: List + 'static> ListSection<T> {
    pub fn lists(mut self, lists: Option<Vec<T>>) -> Self {
        self.lists = lists;
        self
    }

    pub fn view(self) -> Html {
        ui::list::ViewList::default()
            .view()
            .id("list-section")
            .class("w-full flex flex-col")
            .children(match self.lists {
                None => (0..6)
                    .map(|_| ui::list::ViewListItem::default().skeleton(true).view())
                    .collect(),

                Some(lists) => lists
                    .into_iter()
                    .map(|list| {
                        let list = Arc::new(list);
                        ui::list::ViewListItem::default()
                            .title(list.name())
                            .art({
                                let list = list.clone();
                                move |c| list.view_art(&c)
                            })
                            .view()
                            .data_on({
                                let list = list.clone();
                                move |e| e.press_up().push_url(&list.details_url())
                            })
                    })
                    .collect(),
            })
    }
}
