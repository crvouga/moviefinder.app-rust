pub mod random;

use crate::core::pagination::Paginated;
use crate::media::media::Media;

pub trait MediaDb: Send + Sync {
    fn query(&self) -> Result<Paginated<Media>, String>;
    fn put(&self, media: Vec<Media>) -> Result<(), String>;
}
