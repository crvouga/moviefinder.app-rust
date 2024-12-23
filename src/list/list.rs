use super::list_id::MediaListId;
use crate::core::html::Elem;

pub trait MediaList: Clone {
    fn id(&self) -> MediaListId;
    fn view_art(&self, class: &str) -> Elem;
    fn name(&self) -> String;
    fn details_url(&self) -> String;
}
