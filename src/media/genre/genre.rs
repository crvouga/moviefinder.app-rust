use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use super::genre_id::GenreId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Genre {
    pub id: GenreId,
    pub name: String,
}

impl PartialOrd for Genre {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Genre {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}
