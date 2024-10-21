use std::vec;

use async_trait::async_trait;

use crate::core::pagination::Paginated;
use crate::core::random;
use crate::media::media;

use super::MediaDb;

pub struct Random {}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}

#[async_trait]
impl MediaDb for Random {
    async fn query(&self) -> Result<Paginated<media::Media>, String> {
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

        if random::bool() {
            return Err("Something went wrong".to_string());
        }

        Ok(paginated)
    }
}
