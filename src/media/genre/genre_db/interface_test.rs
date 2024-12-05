#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use crate::{
        env::TestEnv,
        fixture::BaseFixture,
        media::genre::genre_db::{impl_tmdb::Tmdb, interface::GenreDb},
    };

    struct Fixture {
        genre_db: Arc<dyn GenreDb + 'static>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let f = BaseFixture::new().await;

        let mut fixtures = vec![];

        if f.env.test_env == TestEnv::Integration {
            let tmdb = Fixture {
                genre_db: Arc::new(Tmdb::new(f.ctx.tmdb_api)),
            };

            fixtures.push(tmdb);
        }

        fixtures
    }

    #[tokio::test]
    async fn test_get_all() {
        for f in fixtures().await {
            let result = f.genre_db.get_all().await.unwrap();
            assert!(result.len() > 0);
        }
    }
}
