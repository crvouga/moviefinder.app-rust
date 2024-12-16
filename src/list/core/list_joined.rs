use super::{list::List, list_item::ListItem};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListJoined {
    pub list: List,
    pub list_items: Vec<ListItem>,
}
