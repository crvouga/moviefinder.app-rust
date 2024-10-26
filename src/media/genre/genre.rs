use super::genre_id::GenreId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Genre {
    pub id: GenreId,
    pub name: String,
}
