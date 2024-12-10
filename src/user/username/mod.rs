use serde::{Deserialize, Serialize};

use crate::core::random;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn generate() -> Self {
        let random_numbers = (random::unit() * 1_000_000.0).floor() as u64;
        let prefix = "movefinder";
        let format = format!("{}{}", prefix, random_numbers);
        Self(format)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn from_string(username: &str) -> Self {
        Self(username.to_string())
    }

    pub fn ensure_valid(self) -> Self {
        self
    }
}
