use super::{tmdb_api, MediaDb, Query};
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
    async fn query(&self, query: &Query) -> Result<Paginated<Media>, String> {
        let config = tmdb_api::config::load(&self.config).await?;

        tmdb_api::discover_movie::send(&self.config)
            .await
            .map(|response| {
                let items = response
                    .results
                    .unwrap_or_default()
                    .into_iter()
                    .map(|result| result.to_media(&config))
                    .collect();
                Paginated {
                    items,
                    limit: query.limit,
                    offset: query.limit,
                    total: response.total_results.unwrap_or(0),
                }
            })
            .map_err(|e| format!("Error fetching from TMDB: {}", e))
    }
}
