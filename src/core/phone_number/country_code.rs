use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumerCountryCode {
    alpha2: String,
    alpha3: String,
    country_code: String,
    country_name: String,
    mobile_begin_with: Vec<String>,
    phone_number_lengths: Vec<u8>,
}

pub trait PhoneNumberCountryCodeDb: Send + Sync {
    fn get_all(&self) -> Vec<PhoneNumerCountryCode>;
}

pub struct ImplJson {
    file_path: String,
}

impl ImplJson {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl PhoneNumberCountryCodeDb for ImplJson {
    fn get_all(&self) -> Vec<PhoneNumerCountryCode> {
        let file_path = &self.file_path;
        let file = std::fs::read_to_string(file_path).unwrap();
        let country_codes: Vec<PhoneNumerCountryCode> = serde_json::from_str(&file).unwrap();
        country_codes
    }
}
