use serde::{Deserialize, Serialize};

pub mod country_code_db;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberCountryCode {
    pub alpha2: String,
    pub alpha3: String,
    pub country_code: String,
    pub country_name: String,
    pub mobile_begin_with: Vec<String>,
    pub phone_number_lengths: Vec<u8>,
}

impl PhoneNumberCountryCode {
    pub fn to_emoji(&self) -> Option<String> {
        let country = self.alpha2.as_str();

        const OFFSET: u32 = 127397;
        const A: u32 = 'A' as u32;
        const Z: u32 = 'Z' as u32;

        if country.len() != 2 {
            return None;
        }

        let mut chars = country.chars();
        let f = chars.next()?.to_ascii_uppercase() as u32;
        let s = chars.next()?.to_ascii_uppercase() as u32;

        if f < A || f > Z || s < A || s > Z {
            return None;
        }

        let emoji = format!(
            "{}{}",
            char::from_u32(f + OFFSET)?,
            char::from_u32(s + OFFSET)?
        );
        Some(emoji)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_emoji() {
        let country_code = PhoneNumberCountryCode {
            alpha2: "US".to_string(),
            alpha3: "USA".to_string(),
            country_code: "1".to_string(),
            country_name: "United States".to_string(),
            mobile_begin_with: vec!["200".to_string()],
            phone_number_lengths: vec![10],
        };

        let emoji = country_code.to_emoji();
        assert_eq!(emoji, Some("🇺🇸".to_string()));
    }
}
