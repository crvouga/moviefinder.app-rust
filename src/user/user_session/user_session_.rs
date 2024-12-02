use crate::{
    core::{posix::Posix, session::session_id::SessionId},
    user::user_id::UserId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub created_at_posix: Posix,
    pub ended_at_posix: Option<Posix>,
}

impl UserSession {
    pub fn new(user_id: &UserId, session_id: &SessionId) -> Self {
        Self {
            user_id: user_id.clone(),
            session_id: session_id.clone(),
            created_at_posix: Posix::now(),
            ended_at_posix: None,
        }
    }
}
