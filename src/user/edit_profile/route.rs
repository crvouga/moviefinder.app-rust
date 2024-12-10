use crate::user::user_id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Screen { user_id: UserId },
    CheckUsernameTaken { user_id: UserId },
    ClickedSave { user_id: UserId },
    ClickedCancel { user_id: UserId },
}
