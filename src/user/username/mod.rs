use serde::{Deserialize, Serialize};

use crate::core::random;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn generate() -> Self {
        let random_numbers = (random::unit() * 1_000_000.0).floor() as u64;
        let prefix = "moviefinder";
        let format = format!("{}{}", prefix, random_numbers);
        Self(format)
    }

    pub fn from_string(s: String) -> Self {
        Self(s.trim().to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
