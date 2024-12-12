use serde::{Deserialize, Serialize};

use crate::ui::route::Routable;

use super::{feed_id::FeedId, feed_tags_form};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    FeedScreenDefault,

    FeedScreen {
        feed_id: FeedId,
    },

    IntersectedBottom {
        feed_id: FeedId,
        bottom_feed_index: usize,
    },

    ChangedSlide {
        feed_id: FeedId,
    },

    Tags(feed_tags_form::route::Route),
}

impl Routable for feed_tags_form::route::Route {
    fn url(&self) -> String {
        Route::Tags(self.clone()).url()
    }
}
