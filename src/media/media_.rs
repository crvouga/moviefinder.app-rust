use std::collections::HashSet;

use super::genre::genre_id::GenreId;
use super::media_id::MediaId;
use super::media_type::MediaType;
use crate::core::image_set::ImageSet;

#[derive(Debug, Clone)]
pub struct Media {
    pub id: MediaId,
    pub title: String,
    pub description: String,
    pub media_type: MediaType,
    pub genre_ids: HashSet<GenreId>,
    pub poster: ImageSet,
    pub backdrop: ImageSet,
    pub popularity: f64,
}

pub fn random() -> Media {
    let mut genre_ids = HashSet::new();
    genre_ids.insert(GenreId::new("random".to_string()));
    Media {
        id: MediaId::new("random".to_string()),
        title: "random".to_string(),
        description: "random".to_string(),
        media_type: MediaType::random(),
        genre_ids,
        poster: ImageSet::empty(),
        backdrop: ImageSet::empty(),
        popularity: 0.0,
    }
}
