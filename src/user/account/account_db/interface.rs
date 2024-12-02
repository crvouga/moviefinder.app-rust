use async_trait::async_trait;

use crate::user::account::account_::Account;

#[async_trait]
pub trait AccountDb {
    async fn find_one_by_phone_number(
        &self,
        phone_number: &str,
    ) -> Result<Option<Account>, std::io::Error>;

    async fn upsert_one(&self, account: &Account) -> Result<(), std::io::Error>;
}
