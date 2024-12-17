use async_trait::async_trait;

use crate::core::phone_number::country_code::PhoneNumberCountryCode;

#[async_trait]
pub trait PhoneNumberCountryCodeDb: Send + Sync {
    async fn get_all(&self) -> Vec<PhoneNumberCountryCode>;
}
