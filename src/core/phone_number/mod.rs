#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod country_code;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PhoneNumber(String);

// to human friendly format
impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = &self.0;
        write!(f, "({}) {}-{}", &s[0..3], &s[3..6], &s[6..10])
    }
}

pub enum PhoneNumberError {
    TooShort,
    TooLong,
}

impl std::fmt::Display for PhoneNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PhoneNumberError::TooShort => write!(f, "Phone number is too short"),
            PhoneNumberError::TooLong => write!(f, "Phone number is too long"),
        }
    }
}

impl PhoneNumber {
    pub fn new(value: &str) -> Result<Self, PhoneNumberError> {
        if value.len() < 10 {
            return Err(PhoneNumberError::TooShort);
        }
        if value.len() > 15 {
            return Err(PhoneNumberError::TooLong);
        }
        Ok(PhoneNumber(value.to_string()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub fn add_country_code(country_code: &str, phone_number: &str) -> String {
    let country_code = country_code.trim();
    let phone_number = phone_number.trim();

    if phone_number.starts_with("+") {
        return phone_number.to_string();
    }

    format!(
        "+{}{}",
        country_code.replace("+", ""),
        phone_number.replace("+", "")
    )
}

pub fn ensure_country_code(
    country_codes: Vec<String>,
    country_code: &str,
    phone_number: &str,
) -> String {
    let fallback_country_code = "1";

    let country_code = if country_code.trim().is_empty() {
        fallback_country_code.trim()
    } else {
        country_code.trim()
    };

    let country_code = country_codes
        .into_iter()
        .find(|c: &String| phone_number.starts_with(c))
        .unwrap_or(country_code.to_string());

    let country_code_without_plus = country_code.strip_prefix("+").unwrap_or(&country_code);

    let country_code_with_plus = format!("+{}", country_code_without_plus);

    let phone_number = phone_number
        .strip_prefix(country_code_without_plus)
        .unwrap_or(phone_number);

    let phone_number_without_country_code = phone_number
        .strip_prefix(&country_code_with_plus)
        .unwrap_or(phone_number);

    let phone_number_with_country_code = format!(
        "{}{}",
        country_code_with_plus, phone_number_without_country_code,
    );

    phone_number_with_country_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_country_code() {
        assert_eq!(
            ensure_country_code(vec!["1".to_string()], "1", "4802098698"),
            "+14802098698"
        );
        assert_eq!(
            ensure_country_code(vec![], "1", "5555555555"),
            "+15555555555"
        );
        assert_eq!(
            ensure_country_code(vec![], "", "5555555555"),
            "+15555555555"
        );
        assert_eq!(
            ensure_country_code(vec![], "1", "+15555555555"),
            "+15555555555"
        );
        assert_eq!(
            ensure_country_code(vec![], "", "+15555555555"),
            "+15555555555"
        );
        assert_eq!(
            ensure_country_code(vec!["1".to_string()], "+1", "15555555555"),
            "+15555555555"
        );
    }
}
