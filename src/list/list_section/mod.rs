use super::list::MediaList;
use crate::core::{html::Elem, ui};
use std::sync::Arc;

#[derive(Clone)]
pub struct ListSection<T: MediaList + 'static> {
    lists: Option<Vec<T>>,
}

impl<T: MediaList + 'static> Default for ListSection<T> {
    fn default() -> Self {
        Self { lists: None }
    }
}

impl<T: MediaList + 'static> ListSection<T> {
    pub fn lists(mut self, lists: Option<Vec<T>>) -> Self {
        self.lists = lists;
        self
    }

    pub fn view(self) -> Elem {
        ui::list::List::default()
            .view()
            .id("list-section")
            .class("w-full flex flex-col")
            .children(match self.lists {
                None => (0..6)
                    .map(|_| ui::list::ListItem::default().skeleton(true).view())
                    .collect(),

                Some(lists) => lists
                    .into_iter()
                    .map(|list| {
                        let list = Arc::new(list);
                        ui::list::ListItem::default()
                            .title(list.name())
                            .art({
                                let list = list.clone();
                                move |c| list.view_art(&c)
                            })
                            .view()
                            .data_on({
                                let list = list.clone();
                                move |e| e.click().push_url(&list.details_url())
                            })
                    })
                    .collect(),
            })
    }
}
