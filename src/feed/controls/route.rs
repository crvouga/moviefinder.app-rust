use serde::{Deserialize, Serialize};

use crate::feed::feed_tag::FeedTag;

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum Route {
    IndexLoad,
    Index,
    ClickedSave,
    InputtedSearch,
    ClickedTag { tag: FeedTag },
    ClickedGoBack,
}
