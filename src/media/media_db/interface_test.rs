#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };

    use crate::{
        core::query::{Filter, Op, Query},
        env,
        fixture::BaseFixture,
        media::{
            media_db::{
                impl_tmdb,
                interface::{MediaDb, MediaField},
            },
            media_id::MediaId,
        },
    };

    struct Fixture {
        pub media_db: Box<dyn MediaDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let base_fixture = BaseFixture::new().await;

        let mut fixtures: Vec<Fixture> = vec![];

        if base_fixture.env.test_env == env::TestEnv::Integration {
            let tmdb_movie = Fixture {
                media_db: Box::new(impl_tmdb::ImplTmdb::new(base_fixture.ctx.tmdb_api.clone())),
            };

            fixtures.push(tmdb_movie);
        }

        fixtures
    }

    #[tokio::test]
    async fn test_find_by_id() {
        for f in fixtures().await {
            let media_id = MediaId::new("123".to_string());

            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::clause(MediaField::MediaId, Op::Eq, media_id.as_str().to_string()),
            };

            let result = f.media_db.query(query).await;

            let first = result.unwrap().items.into_iter().next().unwrap();

            assert_eq!(first.media_id, media_id);
        }
    }

    #[tokio::test]
    async fn test_limit_and_offset() {
        for f in fixtures().await {
            let limit: usize = 40;
            let query = Query {
                limit,
                offset: 0,
                filter: Filter::None,
            };
            let result = f.media_db.query(query).await.unwrap();

            assert_eq!(result.items.len(), limit);
            assert_eq!(result.limit, limit);
            assert_eq!(result.offset, 0);
        }
    }

    #[tokio::test]
    async fn test_no_duplicates() {
        for f in fixtures().await {
            let limit: usize = 50;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset: 0,
                    filter: Filter::None,
                })
                .await
                .unwrap();

            let media_ids: Vec<MediaId> = queried
                .items
                .iter()
                .map(|media| media.media_id.clone())
                .collect();

            let media_id_frequencies = frequencies(media_ids.clone());

            let duplicate_media_ids: Vec<MediaId> = media_ids
                .clone()
                .into_iter()
                .filter(|media_id| media_id_frequencies.get(media_id).unwrap_or(&0) > &1)
                .collect();

            let unique_media_ids: HashSet<MediaId> = media_ids.clone().into_iter().collect();

            assert_eq!(duplicate_media_ids.len(), 0);
            assert_eq!(media_ids.len(), unique_media_ids.len());
        }
    }

    #[tokio::test]
    async fn test_no_duplicates_small_limit_and_offset() {
        for f in fixtures().await {
            let limit: usize = 4;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset: 5,
                    filter: Filter::None,
                })
                .await
                .unwrap();

            let media_ids = queried
                .items
                .iter()
                .map(|media| media.media_id.clone())
                .collect::<Vec<MediaId>>();
            let media_id_frequencies = frequencies(media_ids.clone());

            let duplicate_media_ids = media_ids
                .iter()
                .filter(|media_id| media_id_frequencies.get(media_id).unwrap_or(&0) > &1)
                .collect::<Vec<&MediaId>>();

            let unique_media_ids = media_ids
                .iter()
                .collect::<std::collections::HashSet<&MediaId>>();

            assert_eq!(duplicate_media_ids.len(), 0);
            assert_eq!(media_ids.len(), unique_media_ids.len());
        }
    }

    #[tokio::test]
    async fn test_offset() {
        for f in fixtures().await {
            let limit: usize = 4;
            let offset: usize = 20;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset,
                    filter: Filter::None,
                })
                .await
                .unwrap();

            assert_eq!(queried.items.len(), limit);
        }
    }

    #[tokio::test]
    async fn test_offset_40() {
        for f in fixtures().await {
            let limit: usize = 4;
            let offset: usize = 40;
            let queried = f
                .media_db
                .query(Query {
                    limit,
                    offset,
                    filter: Filter::None,
                })
                .await
                .unwrap();

            assert_eq!(queried.items.len(), limit);
        }
    }

    fn frequencies<T>(items: Vec<T>) -> HashMap<T, usize>
    where
        T: Hash + Eq,
    {
        let mut freq = HashMap::new();

        for item in items {
            *freq.entry(item).or_insert(0) += 1;
        }

        freq
    }
}
