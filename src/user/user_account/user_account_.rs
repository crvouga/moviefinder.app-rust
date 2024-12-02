use serde::{Deserialize, Serialize};

use crate::{core::posix::Posix, user::user_id::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAccount {
    pub user_id: UserId,
    pub phone_number: String,
    pub created_at_posix: Posix,
}

impl UserAccount {
    pub fn new(phone_number: String) -> Self {
        Self {
            user_id: UserId::default(),
            phone_number,
            created_at_posix: Posix::now(),
        }
    }
}
