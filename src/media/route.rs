use serde::{Deserialize, Serialize};

use crate::ui::to_url::ToURL;

use super::details;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Details(details::route::Route),
}

impl ToURL for details::route::Route {
    fn to_url(&self) -> String {
        Route::Details(self.clone()).to_url()
    }
}
