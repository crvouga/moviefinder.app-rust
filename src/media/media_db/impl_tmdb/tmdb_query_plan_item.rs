use crate::{
    core::pagination::{PageBased, Paginated},
    media::{
        core::Media,
        media_db::interface::MediaQuery,
        media_id::MediaId,
        tmdb_api::{
            self, config::TmdbConfig, discover_movie::DiscoverMovieParams, TmdbApi, TMDB_PAGE_SIZE,
        },
    },
};
use futures::future::join_all;
use std::vec;

#[derive(Debug, Clone)]
pub enum TmdbQueryPlanItem {
    GetMovieDetails { media_id: MediaId },
    GetDiscoverMovie(GetDiscoverMovieParams),
}

#[derive(Debug, Clone)]
pub struct GetDiscoverMovieParams {
    page_based: PageBased,
    params: Vec<DiscoverMovieParams>,
    limit: usize,
}

impl TmdbQueryPlanItem {
    pub async fn execute(
        &self,
        tmdb_api: &TmdbApi,
        tmdb_config: &TmdbConfig,
    ) -> Result<Paginated<Media>, String> {
        match self {
            TmdbQueryPlanItem::GetMovieDetails { media_id } => {
                let movie_details_response = tmdb_api.movie_details(media_id.as_str()).await?;

                let movie = Media::from((tmdb_config, movie_details_response));

                Ok(Paginated {
                    items: vec![movie],
                    limit: 1,
                    offset: 0,
                    total: 1,
                })
            }
            TmdbQueryPlanItem::GetDiscoverMovie(GetDiscoverMovieParams {
                limit,
                page_based,
                params,
            }) => {
                let discover_requests = params
                    .iter()
                    .map(|params| tmdb_api.discover_movie(params.clone()));

                let discover_responses: Vec<tmdb_api::discover_movie::DiscoverMovieResponse> =
                    partition_results(join_all(discover_requests).await).unwrap_or_default();

                let offset = page_based.index + 1;

                let items = discover_responses
                    .clone()
                    .into_iter()
                    .flat_map(|res| res.results.unwrap_or_default())
                    .skip(offset)
                    .take(limit.clone())
                    .map(|result| Media::from((tmdb_config, result)))
                    .collect();

                let total = discover_responses
                    .into_iter()
                    .map(|res| res.total_results.unwrap_or(0))
                    .max()
                    .unwrap_or(0);

                Ok(Paginated {
                    items,
                    total,
                    limit: limit.clone(),
                    offset,
                })
            }
        }
    }
}

impl Into<GetDiscoverMovieParams> for MediaQuery {
    fn into(self) -> GetDiscoverMovieParams {
        let mut params = vec![];

        let page_based: PageBased = (&self, TMDB_PAGE_SIZE).into();

        for page in page_based.range() {
            let param = DiscoverMovieParams {
                page: Some(page as usize),
                ..Default::default()
            };

            params.push(param);
        }

        GetDiscoverMovieParams {
            limit: self.limit,
            params,
            page_based,
        }
    }
}

fn partition_results<T, E>(results: Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let mut oks = Vec::new();
    let mut errs = Vec::new();

    for result in results {
        match result {
            Ok(val) => oks.push(val),
            Err(err) => errs.push(err),
        }
    }

    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(errs)
    }
}
