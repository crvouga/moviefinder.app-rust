use serde::{Deserialize, Serialize};

use crate::{core::posix::Posix, user::user_id::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: UserId,
    pub username: String,
    pub created_at_posix: Posix,
}
