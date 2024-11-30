use serde::{Deserialize, Serialize};

use crate::feed::feed_id::FeedId;

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum Route {
    Index { feed_id: FeedId },
    ClickedSave { feed_id: FeedId },
    InputtedSearch { feed_id: FeedId },
    ClickedGoBack { feed_id: FeedId },
    ClickedTag { feed_id: FeedId },
}
