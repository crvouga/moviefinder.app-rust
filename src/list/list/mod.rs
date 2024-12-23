use super::list_id::ListId;
use crate::core::html::Elem;

pub mod list_section;

pub trait List: Clone {
    fn id(&self) -> ListId;
    fn view_art(&self, class: &str) -> Elem;
    fn name(&self) -> String;
    fn details_url(&self) -> String;
}
