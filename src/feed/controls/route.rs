use serde::{Deserialize, Serialize};

use crate::feed::{feed_id::FeedId, feed_tag::FeedTag};

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum Route {
    IndexLoad { feed_id: FeedId },
    Index { feed_id: FeedId },
    ClickedSave { feed_id: FeedId },
    InputtedSearch { feed_id: FeedId },
    ClickedTag { feed_id: FeedId, tag: FeedTag },
    ClickedGoBack { feed_id: FeedId },
}
