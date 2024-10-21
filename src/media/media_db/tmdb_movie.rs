use crate::core::pagination::Paginated;
use crate::media::media;

use super::{tmdb_api, MediaDb};

pub struct TmdbMovie {
    config: tmdb_api::Config,
}

impl TmdbMovie {
    pub fn new(tmdb_api_read_access_token: String) -> TmdbMovie {
        TmdbMovie {
            config: tmdb_api::Config::new(tmdb_api_read_access_token),
        }
    }
}

impl MediaDb for TmdbMovie {
    fn query(&self) -> Result<Paginated<media::Media>, String> {
        let sent = tmdb_api::discover_movie::send(&self.config);

        let paginated = Paginated {
            items: vec![],
            limit: 3,
            offset: 0,
            total: 3,
        };

        Ok(paginated)
    }
}
