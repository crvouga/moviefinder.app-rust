use super::interface::UserAccountDb;
use crate::{
    core::{
        key_value_db::interface::{KeyValueDbDyn, KeyValueDbExt},
        unit_of_work::UnitOfWork,
    },
    user::{user_account::user_account_::UserAccount, user_id::UserId},
};
use async_trait::async_trait;

pub struct KeyValueDb {
    account_by_user_id: KeyValueDbDyn,
    user_id_by_phone_number: KeyValueDbDyn,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbDyn) -> Self {
        Self {
            account_by_user_id: key_value_db
                .clone()
                .namespace(vec!["user".to_string(), "account".to_string()])
                .into(),

            user_id_by_phone_number: key_value_db
                .namespace(vec!["user".to_string(), "user_id".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl UserAccountDb for KeyValueDb {
    async fn find_one_by_phone_number(
        &self,
        phone_number: &str,
    ) -> Result<Option<UserAccount>, crate::core::error::CoreError> {
        let maybe_user_id = self
            .user_id_by_phone_number
            .get::<UserId>(phone_number)
            .await
            .unwrap_or_default();

        let user_id = match maybe_user_id {
            Some(id) => id,
            None => return Ok(None),
        };

        self.find_one_by_user_id(&user_id).await
    }

    async fn find_one_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserAccount>, crate::core::error::CoreError> {
        Ok(self
            .account_by_user_id
            .get::<UserAccount>(user_id.as_str())
            .await
            .unwrap_or(None))
    }

    async fn put(
        &self,
        uow: UnitOfWork,
        account: &UserAccount,
    ) -> Result<(), crate::core::error::CoreError> {
        let user_id = account.user_id.as_str().to_string();
        let phone_number = account.phone_number.clone();

        self.account_by_user_id
            .put(uow.clone(), &user_id, account)
            .await?;

        self.user_id_by_phone_number
            .put(uow, &phone_number, &user_id)
            .await?;

        Ok(())
    }
}
