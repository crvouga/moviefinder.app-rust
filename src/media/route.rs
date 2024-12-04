use serde::{Deserialize, Serialize};

use crate::ui::route::Routable;

use super::details;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Details(details::route::Route),
}

impl Routable for details::route::Route {
    fn url(&self) -> String {
        Route::Details(self.clone()).url()
    }
}
