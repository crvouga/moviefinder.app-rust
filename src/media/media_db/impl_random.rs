use super::interface::{MediaDb, MediaQuery};
use crate::{core::pagination::Paginated, core::random, media::media_};
use async_trait::async_trait;
use std::vec;

pub struct Random {}

impl Random {
    pub fn new() -> Random {
        Random {}
    }
}

#[async_trait]
impl MediaDb for Random {
    async fn query(&self, _query: MediaQuery) -> Result<Paginated<media_::Media>, String> {
        let paginated = Paginated {
            items: vec![
                media_::random(),
                media_::random(),
                media_::random(),
                media_::random(),
                media_::random(),
                media_::random(),
                media_::random(),
                media_::random(),
                media_::random(),
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
