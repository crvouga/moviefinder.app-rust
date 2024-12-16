use super::{details, interaction::interaction_form};
use crate::ui::route::AppRoute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    InteractionForm(interaction_form::route::Route),
    Details(details::route::Route),
}

impl AppRoute for details::route::Route {
    fn url(&self) -> String {
        Route::Details(self.clone()).url()
    }
}

impl AppRoute for interaction_form::route::Route {
    fn url(&self) -> String {
        Route::InteractionForm(self.clone()).url()
    }
}
