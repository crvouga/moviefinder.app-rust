use crate::{list::list_screen, user::user_id::UserId};
use serde::{Deserialize, Serialize};

use super::list_::MediaInteractionList;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    ListsSection { user_id: UserId },
    ListScreen(list_screen::route::Route<MediaInteractionList>),
}
