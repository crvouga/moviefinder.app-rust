use async_trait::async_trait;

use crate::{
    core::unit_of_work::UnitOfWork,
    user::{user_id::UserId, user_profile::user_profile_::UserProfile},
};

#[async_trait]
pub trait UserProfileDb: Send + Sync {
    async fn find_one_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserProfile>, crate::core::error::CoreError>;
    async fn put(
        &self,
        uow: UnitOfWork,
        profile: &UserProfile,
    ) -> Result<(), crate::core::error::CoreError>;
}
