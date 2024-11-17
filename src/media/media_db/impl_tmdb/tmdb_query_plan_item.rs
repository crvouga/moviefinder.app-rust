use crate::{
    core::{
        pagination::{PageBased, Paginated},
        query::{QueryFilter, QueryOp},
    },
    media::{
        core::Media,
        media_db::interface::{MediaField, MediaQuery},
        media_id::MediaId,
        tmdb_api::{
            self,
            config::TmdbConfig,
            discover_movie::{DiscoverMovieParams, TMDB_AND_OP, TMDB_OR_OP},
            TmdbApi, TMDB_PAGE_SIZE,
        },
    },
};
use futures::future::join_all;
use std::{collections::HashSet, vec};

#[derive(Debug, Clone)]
pub enum TmdbQueryPlanItem {
    GetMovieDetails { media_id: MediaId },
    GetDiscoverMovie { params: GetDiscoverMovieParams },
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
            TmdbQueryPlanItem::GetDiscoverMovie { params } => {
                let discover_requests = params
                    .params
                    .iter()
                    .map(|params| tmdb_api.discover_movie(params.clone()));

                let discover_responses: Vec<tmdb_api::discover_movie::DiscoverMovieResponse> =
                    partition_results(join_all(discover_requests).await).unwrap_or_default();

                let offset = params.page_based.index;

                let mut seen: HashSet<MediaId> = HashSet::new();
                let items = discover_responses
                    .clone()
                    .into_iter()
                    .flat_map(|res| res.results.unwrap_or_default())
                    .map(|result| Media::from((tmdb_config, result)))
                    .filter(|media| seen.insert(media.media_id.clone()))
                    .skip(offset)
                    .take(params.limit.clone())
                    .collect();

                let total = discover_responses
                    .into_iter()
                    .map(|res| res.total_results.unwrap_or(0))
                    .max()
                    .unwrap_or(0);

                Ok(Paginated {
                    items,
                    total,
                    limit: params.limit.clone(),
                    offset,
                })
            }
        }
    }
}

impl From<MediaQuery> for GetDiscoverMovieParams {
    fn from(media_query: MediaQuery) -> GetDiscoverMovieParams {
        let mut params = vec![];

        let page_based: PageBased = (&media_query, TMDB_PAGE_SIZE).into();

        for page in page_based.start_page..=(page_based.end_page + 1) {
            let param = DiscoverMovieParams {
                page: Some(page as usize),
                ..media_query.clone().into()
            };

            params.push(param);
        }

        GetDiscoverMovieParams {
            limit: media_query.limit,
            params,
            page_based,
        }
    }
}

// https://developer.themoviedb.org/reference/discover-movie
impl From<MediaQuery> for DiscoverMovieParams {
    fn from(media_query: MediaQuery) -> DiscoverMovieParams {
        let mut params = DiscoverMovieParams {
            ..Default::default()
        };

        match media_query.filter {
            QueryFilter::Clause(MediaField::GenreId, QueryOp::Eq, value) => {
                params.with_genres = Some(value);
            }
            QueryFilter::And(filters) => {
                for filter in filters {
                    match filter {
                        QueryFilter::Clause(MediaField::GenreId, QueryOp::Eq, value) => {
                            let with_genres_new = format!(
                                "{}{}{}",
                                params.with_genres.unwrap_or_default(),
                                TMDB_AND_OP,
                                value
                            );

                            let cleaned = remove_prefix(with_genres_new, TMDB_AND_OP);

                            params.with_genres = Some(cleaned)
                        }
                        _ => {}
                    }
                }
            }
            QueryFilter::Or(filters) => {
                for filter in filters {
                    match filter {
                        QueryFilter::Clause(MediaField::GenreId, QueryOp::Eq, value) => {
                            let with_genres_new = format!(
                                "{}{}{}",
                                params.with_genres.unwrap_or_default(),
                                TMDB_OR_OP,
                                value
                            );

                            let cleaned = remove_prefix(with_genres_new, TMDB_OR_OP);

                            params.with_genres = Some(cleaned)
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        params
    }
}

fn remove_prefix(s: String, prefix: &str) -> String {
    if s.starts_with(prefix) {
        s[prefix.len()..].to_string()
    } else {
        s
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
