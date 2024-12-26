use super::list_id::ListId;
use crate::core::html::Html;

pub trait List: Clone {
    fn id(&self) -> ListId;
    fn view_art(&self, class: &str) -> Html;
    fn name(&self) -> String;
    fn details_url(&self) -> String;
}
