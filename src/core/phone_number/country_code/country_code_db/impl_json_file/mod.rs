use async_trait::async_trait;

use super::interface::PhoneNumberCountryCodeDb;
use crate::core::phone_number::country_code::PhoneNumberCountryCode;
use std::path::PathBuf;

pub struct ImplJsonFile {
    file_path: PathBuf,
}

impl ImplJsonFile {
    pub fn new() -> Self {
        let file_path = "./src/core/phone_number/country_code/country_code_db/impl_json_file/country_codes.json"
            .parse::<PathBuf>()
            .unwrap();

        Self { file_path }
    }
}

#[async_trait]
impl PhoneNumberCountryCodeDb for ImplJsonFile {
    async fn get_all(&self) -> Vec<PhoneNumberCountryCode> {
        let file = std::fs::read_to_string(&self.file_path).unwrap();
        let country_codes: Vec<PhoneNumberCountryCode> = serde_json::from_str(&file).unwrap();
        country_codes
    }
}
