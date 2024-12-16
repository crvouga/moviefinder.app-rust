use crate::{ui::route::AppRoute, user::user_id::UserId};
use serde::{Deserialize, Serialize};

use super::avatar;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Screen { user_id: UserId },
    InputtedUsername { user_id: UserId },
    SubmittedForm { user_id: UserId },
    Avatar(avatar::route::Route),
}

impl AppRoute for avatar::route::Route {
    fn url(&self) -> String {
        Route::Avatar(self.clone()).url()
    }
}
