use std::{collections::HashMap, hash::Hash};

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::Hash};

    use crate::{
        core::query::{Filter, Op, Query},
        env,
        media::{
            media_db::{
                impl_tmdb,
                interface::{Field, MediaDb},
            },
            media_id::MediaId,
        },
    };

    struct Fixture {
        pub media_db: Box<dyn MediaDb>,
    }

    fn fixtures() -> Vec<Fixture> {
        let env = match env::Env::load() {
            Some(env) => env,
            None => return vec![],
        };

        let mut fixtures: Vec<Fixture> = vec![];

        if env.test_env == env::TestEnv::Integration {
            let tmdb_movie = Fixture {
                media_db: Box::new(impl_tmdb::Tmdb::new(env.tmdb_api_read_access_token)),
            };

            fixtures.push(tmdb_movie);
        }

        fixtures
    }

    #[tokio::test]
    async fn test_find_by_id() {
        for f in fixtures() {
            let media_id = MediaId::new("123".to_string());

            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::clause(Field::MediaId, Op::Eq, media_id.as_str().to_string()),
            };

            let result = f.media_db.query(query).await;

            let first = result.unwrap().items.into_iter().next().unwrap();

            assert_eq!(first.media_id, media_id);
        }
    }

    #[tokio::test]
    async fn test_limit_and_offset() {
        for f in fixtures() {
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
        for f in fixtures() {
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

            // println!("media_ids: {:?}", media_ids);
            println!("media_id_frequencies: {:?}", media_id_frequencies);
            assert_eq!(duplicate_media_ids.len(), 0);
            assert_eq!(media_ids.len(), unique_media_ids.len());
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
