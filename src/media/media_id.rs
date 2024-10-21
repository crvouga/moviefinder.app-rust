use crate::core;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MediaId(String);

impl MediaId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn generate() -> Self {
        Self::new(core::uuid::v4().to_string())
    }
}
