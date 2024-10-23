#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenreId(String);

impl GenreId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
