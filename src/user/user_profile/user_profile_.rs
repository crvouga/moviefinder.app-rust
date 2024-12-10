use serde::{Deserialize, Serialize};

use crate::{
    core::posix::Posix,
    user::{user_id::UserId, username::Username},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: UserId,
    pub username: Username,
    pub created_at_posix: Posix,
}

impl UserProfile {
    pub fn new(user_id: &UserId) -> Self {
        Self {
            user_id: user_id.clone(),
            username: Username::generate(),
            created_at_posix: Posix::now(),
        }
    }
}
