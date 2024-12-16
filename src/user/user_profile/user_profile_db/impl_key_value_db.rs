use super::interface::UserProfileDb;
use crate::{
    core::{key_value_db::interface::KeyValueDbRef, unit_of_work::UnitOfWork},
    user::{user_id::UserId, user_profile::user_profile_::UserProfile},
};
use async_trait::async_trait;

pub struct KeyValueDb {
    profile_by_user_id: KeyValueDbRef,
    user_id_by_username: KeyValueDbRef,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbRef) -> Self {
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
    ) -> Result<Option<UserProfile>, std::io::Error> {
        let maybe_profile = self.profile_by_user_id.get(&user_id.as_str()).await?;

        let profile = match maybe_profile {
            Some(profile) => profile,
            None => return Ok(None),
        };

        let parsed = serde_json::from_str::<UserProfile>(&profile)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))?;

        Ok(Some(parsed))
    }

    async fn upsert_one(
        &self,
        uow: UnitOfWork,
        profile: &UserProfile,
    ) -> Result<(), std::io::Error> {
        let user_id = profile.user_id.as_str().to_string();
        let username = profile.username.clone();

        let serialized = serde_json::to_string(profile)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))?;

        self.profile_by_user_id
            .put(uow.clone(), &user_id, serialized.to_string())
            .await?;

        self.user_id_by_username
            .put(uow, &username.to_string(), user_id.to_string())
            .await?;

        Ok(())
    }
}
