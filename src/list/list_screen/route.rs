use crate::list::list::List;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route<TList: List + Clone + 'static> {
    Screen { list: TList, back_url: String },
    IntersectedBottom { list: TList },
}
