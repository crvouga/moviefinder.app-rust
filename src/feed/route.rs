use super::{feed_screen, feed_tags_form};
use crate::ui::route::AppRoute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    FeedScreen(feed_screen::route::Route),
    Tags(feed_tags_form::route::Route),
}

impl AppRoute for feed_screen::route::Route {
    fn url(&self) -> String {
        Route::FeedScreen(self.clone()).url()
    }
}

impl AppRoute for feed_tags_form::route::Route {
    fn url(&self) -> String {
        Route::Tags(self.clone()).url()
    }
}
