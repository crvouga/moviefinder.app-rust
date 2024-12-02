use async_trait::async_trait;

use crate::user::{profile::profile_::Profile, user_id::UserId};

#[async_trait]
pub trait UserProfileDb: Send + Sync {
    async fn find_one_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<Profile>, std::io::Error>;
    async fn upsert_one(&self, profile: &Profile) -> Result<(), std::io::Error>;
}
