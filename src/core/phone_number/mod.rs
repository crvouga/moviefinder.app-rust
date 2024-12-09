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
