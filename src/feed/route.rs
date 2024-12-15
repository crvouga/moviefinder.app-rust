use serde::{Deserialize, Serialize};

use crate::ui::route::Routable;

use super::{feed_screen, feed_tags_form};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    FeedScreen(feed_screen::route::Route),
    Tags(feed_tags_form::route::Route),
}

impl Routable for feed_screen::route::Route {
    fn url(&self) -> String {
        Route::FeedScreen(self.clone()).url()
    }
}

impl Routable for feed_tags_form::route::Route {
    fn url(&self) -> String {
        Route::Tags(self.clone()).url()
    }
}
