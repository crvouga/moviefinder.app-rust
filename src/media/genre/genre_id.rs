#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenreId(String);

impl GenreId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn gen() -> Self {
        Self::new("gen".to_string())
    }
}
