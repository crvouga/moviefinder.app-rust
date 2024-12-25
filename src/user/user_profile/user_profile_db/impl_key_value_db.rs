use super::interface::UserProfileDb;
use crate::{
    core::{
        key_value_db::interface::{KeyValueDbDyn, KeyValueDbExt},
        unit_of_work::UnitOfWork,
    },
    user::{user_id::UserId, user_profile::user_profile_::UserProfile},
};
use async_trait::async_trait;

pub struct KeyValueDb {
    profile_by_user_id: KeyValueDbDyn,
    user_id_by_username: KeyValueDbDyn,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbDyn) -> Self {
        Self {
            profile_by_user_id: key_value_db
                .clone()
                .child(vec!["profile_by_user_id".to_string()])
                .into(),

            user_id_by_username: key_value_db
                .child(vec!["user_id_by_username".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl UserProfileDb for KeyValueDb {
    async fn find_one_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserProfile>, crate::core::error::Error> {
        self.profile_by_user_id.get(&user_id.as_str()).await
    }

    async fn put(
        &self,
        uow: UnitOfWork,
        profile: &UserProfile,
    ) -> Result<(), crate::core::error::Error> {
        let user_id = profile.user_id.as_str().to_string();
        let username = profile.username.clone();

        self.profile_by_user_id
            .put(uow.clone(), &user_id, profile)
            .await?;

        self.user_id_by_username
            .put(uow, &username.to_string(), &user_id)
            .await?;

        Ok(())
    }
}
