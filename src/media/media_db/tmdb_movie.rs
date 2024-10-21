use super::{tmdb_api, MediaDb};
use crate::{core::pagination::Paginated, media::media::Media};
use async_trait::async_trait;

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

#[async_trait]
impl MediaDb for TmdbMovie {
    async fn query(&self) -> Result<Paginated<Media>, String> {
        let sent: Result<tmdb_api::discover_movie::DiscoverMovieResponse, String> =
            tmdb_api::discover_movie::send(&self.config).await;

        match sent {
            Ok(_response) => {
                let paginated = Paginated {
                    items: vec![],
                    limit: 0,
                    offset: 0,
                    total: 0,
                };

                Ok(paginated)
            }
            Err(e) => Err(format!("Error fetching from TMDB: {}", e)),
        }
    }
}
