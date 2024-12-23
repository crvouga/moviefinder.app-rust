use async_trait::async_trait;

use crate::core::session::session_id::SessionId;
use crate::core::unit_of_work::UnitOfWork;
use crate::user::user_session::user_session_::UserSession;

#[async_trait]
pub trait UserSessionDb: Send + Sync {
    // async fn find_by_user_id(
    //     &self,
    //     user_id: &UserId,
    // ) -> Result<Option<UserSession>, crate::core::error::Error>;
    async fn find_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<UserSession>, crate::core::error::Error>;
    async fn put(
        &self,
        uow: UnitOfWork,
        session: &UserSession,
    ) -> Result<(), crate::core::error::Error>;
    async fn zap(
        &self,
        uow: UnitOfWork,
        session_id: &SessionId,
    ) -> Result<(), crate::core::error::Error>;
}
