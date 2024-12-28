use crate::{
    core::{
        cache_db::{cached_query::CachedQuery, interface::CacheDbDyn},
        pagination::{PageBased, Paginated},
        query::{QueryFilter, QueryOp},
        tmdb_api::{
            config::TmdbConfig,
            discover_movie::{
                DiscoverMovieParams, DiscoverMovieResponse, DiscoverMovieResult, TMDB_AND_OP,
                TMDB_OR_OP,
            },
            TmdbApi, TMDB_PAGE_SIZE,
        },
        unit_of_work::uow,
    },
    media::{
        genre::genre_id::GenreId,
        media_::Media,
        media_db::interface::{MediaQuery, MediaQueryField},
        media_id::MediaId,
        media_type::MediaType,
    },
};
use std::{collections::HashSet, sync::Arc, time::Duration, vec};

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
        cache_db: CacheDbDyn,
        tmdb_api: Arc<TmdbApi>,
        tmdb_config: Arc<TmdbConfig>,
    ) -> Result<Paginated<Media>, crate::core::error::Error> {
        match self {
            TmdbQueryPlanItem::GetMovieDetails { media_id } => {
                let media_id_owned = media_id.clone();

                let tmdb_api_clone = Arc::clone(&tmdb_api);
                let cache_db_clone = cache_db.clone();

                let tmdb_movie_details = CachedQuery::new()
                    .cache_db(cache_db_clone)
                    .key(format!("movie_details_{}", media_id.as_str()))
                    .strategy_strictly_fresh()
                    .max_age(Duration::from_secs(60 * 60 * 24)) // 24 hours
                    .uow(uow())
                    .query(move || {
                        let tmdb_api = Arc::clone(&tmdb_api_clone);
                        let media_id = media_id_owned.clone();
                        async move {
                            tmdb_api
                                .movie_details(media_id.as_str())
                                .await
                                .map_err(|e| crate::core::error::Error::new(e))
                        }
                    })
                    .execute()
                    .await
                    .to_result()?;

                let movie = Media::from((tmdb_config.as_ref(), tmdb_movie_details));

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
                    .clone()
                    .into_iter()
                    .map(|param| tmdb_api.discover_movie(param));

                let mut discover_responses: Vec<DiscoverMovieResponse> = vec![];

                for request in discover_requests {
                    let result = request.await;
                    match result {
                        Ok(val) => discover_responses.push(val),
                        Err(err) => return Err(crate::core::error::Error::new(err)),
                    }
                }

                let offset = params.page_based.index;

                let mut seen: HashSet<MediaId> = HashSet::new();
                let items = discover_responses
                    .clone()
                    .into_iter()
                    .flat_map(|res| res.results.unwrap_or_default())
                    .map(|result| Media::from((tmdb_config.as_ref(), result)))
                    .filter(|media| seen.insert(media.id.clone()))
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

        let page_based = media_query.to_page_based(TMDB_PAGE_SIZE);

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
            QueryFilter::Clause(MediaQueryField::GenreId, QueryOp::Eq, value) => {
                params.with_genres = Some(value);
            }
            QueryFilter::And(filters) => {
                for filter in filters {
                    match filter {
                        QueryFilter::Clause(MediaQueryField::GenreId, QueryOp::Eq, value) => {
                            let with_genres_new = format!(
                                "{}{}{}",
                                params.with_genres.unwrap_or_default(),
                                TMDB_AND_OP,
                                value
                            );

                            let cleaned = remove_prefix(with_genres_new, TMDB_AND_OP);

                            params.with_genres = Some(cleaned)
                        }
                        QueryFilter::Clause(MediaQueryField::PersonId, QueryOp::Eq, value) => {
                            let with_cast_new = format!(
                                "{}{}{}",
                                params.with_cast.unwrap_or_default(),
                                TMDB_AND_OP,
                                value
                            );

                            let cleaned = remove_prefix(with_cast_new, TMDB_AND_OP);

                            params.with_cast = Some(cleaned)
                        }
                        _ => {}
                    }
                }
            }
            QueryFilter::Or(filters) => {
                for filter in filters {
                    match filter {
                        QueryFilter::Clause(MediaQueryField::GenreId, QueryOp::Eq, value) => {
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

impl From<(&TmdbConfig, DiscoverMovieResult)> for Media {
    fn from((config, result): (&TmdbConfig, DiscoverMovieResult)) -> Self {
        Media {
            id: MediaId::new(result.id.unwrap_or(0.0).to_string()),
            backdrop: config
                .to_backdrop_image_set(result.backdrop_path.unwrap_or("".to_string()).as_str()),
            description: result.overview.unwrap_or("".to_string()),
            genre_ids: result
                .genre_ids
                .unwrap_or(vec![])
                .iter()
                .map(|id| GenreId::new(id.to_string()))
                .collect(),
            popularity: result.popularity.unwrap_or(0.0),
            poster: config
                .to_poster_image_set(result.poster_path.unwrap_or("".to_string()).as_str()),
            title: result.title.unwrap_or("".to_string()),
            media_type: MediaType::Movie,
        }
    }
}
