#[cfg(test)]
mod tests {
    use crate::{
        core::query::{Filter, Operator, Query},
        env,
        media::{
            media_db::{impl_tmdb_movie, Field, MediaDb},
            media_id::MediaId,
        },
    };

    struct Fixture {
        pub media_db: Box<dyn MediaDb>,
    }

    fn fixtures() -> Vec<Fixture> {
        let env = env::Env::load();

        let mut fixtures: Vec<Fixture> = vec![];

        if env.test_env == env::TestEnv::Integration {
            let tmdb_movie = Fixture {
                media_db: Box::new(impl_tmdb_movie::TmdbMovie::new(
                    env.tmdb_api_read_access_token,
                )),
            };

            fixtures.push(tmdb_movie);
        }

        fixtures
    }

    #[tokio::test]
    async fn test_find_by_id() {
        for f in fixtures() {
            let media_id = MediaId::new("1".to_string());

            let query = Query {
                limit: 1,
                offset: 0,
                filter: Filter::clause(Field::MediaId, Operator::Eq, media_id.as_str().to_string()),
            };

            let result = f.media_db.query(&query).await;

            let first = result.unwrap().items.into_iter().next().unwrap();

            assert_eq!(first.media_id, media_id);
        }
    }
}
