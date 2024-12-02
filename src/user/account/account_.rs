use serde::{Deserialize, Serialize};

use crate::user::user_id::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub user_id: UserId,
    pub phone_number: String,
    pub created_at_posix: i64,
}
