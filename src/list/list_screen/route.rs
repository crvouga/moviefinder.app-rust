use crate::list::list::MediaList;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route<TList: MediaList + Clone + 'static> {
    Screen { list: TList, back_url: String },
    IntersectedBottom { list: TList },
}
