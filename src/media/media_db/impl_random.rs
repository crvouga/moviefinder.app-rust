use super::interface::{Field, MediaDb};
use crate::{core::pagination::Paginated, core::query::Query, core::random, media::core};
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
    async fn query(&self, _query: Query<Field>) -> Result<Paginated<core::Media>, String> {
        let paginated = Paginated {
            items: vec![
                core::random(),
                core::random(),
                core::random(),
                core::random(),
                core::random(),
                core::random(),
                core::random(),
                core::random(),
                core::random(),
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
