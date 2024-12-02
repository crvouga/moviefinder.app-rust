use async_trait::async_trait;

use crate::core::session::session_id::SessionId;
use crate::user::{user_id::UserId, user_session::user_session_::UserSession};

#[async_trait]
pub trait UserSessionDb: Send + Sync {
    async fn find_one_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserSession>, std::io::Error>;
    async fn find_one_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<UserSession>, std::io::Error>;
    async fn upsert_one(&self, session: &UserSession) -> Result<(), std::io::Error>;
}
