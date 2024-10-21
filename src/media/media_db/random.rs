use std::vec;

use crate::core::pagination::Paginated;
use crate::media::media;

use super::MediaDb;

pub struct Random {}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}

impl MediaDb for Random {
    fn query(&self) -> Result<Paginated<media::Media>, String> {
        let paginated = Paginated {
            items: vec![
                media::random(),
                media::random(),
                media::random(),
                media::random(),
                media::random(),
                media::random(),
                media::random(),
                media::random(),
                media::random(),
            ],
            limit: 3,
            offset: 0,
            total: 3,
        };

        Ok(paginated)
    }
}
