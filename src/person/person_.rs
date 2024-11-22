use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Person {
    pub name: String,
    pub id: String,
}
