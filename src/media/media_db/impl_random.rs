use std::vec;

use async_trait::async_trait;

use super::{Field, MediaDb};
use crate::{core::pagination::Paginated, core::query::Query, core::random, media::media};

pub struct Random {}

impl Random {}

#[async_trait]
impl MediaDb for Random {
    async fn query(&self, _query: &Query<Field>) -> Result<Paginated<media::Media>, String> {
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
