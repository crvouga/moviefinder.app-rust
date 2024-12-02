use async_trait::async_trait;

use crate::user::user_account::user_account_::UserAccount;

#[async_trait]
pub trait UserAccountDb: Send + Sync {
    async fn find_one_by_phone_number(
        &self,
        phone_number: &str,
    ) -> Result<Option<UserAccount>, std::io::Error>;

    // async fn find_one_by_user_id(
    //     &self,
    //     user_id: &UserId,
    // ) -> Result<Option<UserAccount>, std::io::Error>;

    async fn upsert_one(&self, account: &UserAccount) -> Result<(), std::io::Error>;
}
