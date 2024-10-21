pub mod random;
mod tmdb_api;
pub mod tmdb_movie;

use crate::core::pagination::Paginated;
use crate::media::media::Media;

pub trait MediaDb: Send + Sync {
    fn query(&self) -> Result<Paginated<Media>, String>;
    // fn put(&self, media: Vec<Media>) -> Result<(), String>;
}
