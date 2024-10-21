use super::genre::genre_id::GenreId;
use super::media_id::MediaId;
use super::media_type::MediaType;
use crate::core::image_set::ImageSet;

pub struct Media {
    pub media_id: MediaId,
    pub media_title: String,
    pub media_description: String,
    pub media_type: MediaType,
    pub media_genre_ids: Vec<GenreId>,
    pub media_poster: ImageSet,
    pub media_backdrop: ImageSet,
    pub media_popularity: f64,
}

pub fn random() -> Media {
    Media {
        media_id: MediaId::new("random".to_string()),
        media_title: "random".to_string(),
        media_description: "random".to_string(),
        media_type: MediaType::random(),
        media_genre_ids: vec![GenreId::new("random".to_string())],
        media_poster: ImageSet::empty(),
        media_backdrop: ImageSet::empty(),
        media_popularity: 0.0,
    }
}
