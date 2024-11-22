use crate::{
    env,
    fixture::BaseFixture,
    media::{
        genre::{genre_db::interface::GenreDb, genre_id::GenreId},
        media_db::{impl_tmdb, interface::MediaDb},
    },
};
use std::sync::Arc;

pub struct Fixture {
    pub media_db: Box<dyn MediaDb>,
    pub genre_db: Arc<dyn GenreDb>,
}

impl Fixture {
    pub async fn random_genre_id(&self) -> GenreId {
        let genre_ids: Vec<GenreId> = self
            .genre_db
            .get_all()
            .await
            .unwrap()
            .iter()
            .map(|g| g.id.clone())
            .collect();

        genre_ids.first().cloned().unwrap()
    }
}

pub async fn fixtures() -> Vec<Fixture> {
    let base_fixture = BaseFixture::new().await;
    let mut fixtures: Vec<Fixture> = vec![];

    if base_fixture.env.test_env == env::TestEnv::Integration {
        let fixture = Fixture {
            media_db: Box::new(impl_tmdb::ImplTmdb::new(
                base_fixture.ctx.logger.clone(),
                base_fixture.ctx.tmdb_api.clone(),
            )),
            genre_db: base_fixture.ctx.genre_db,
        };

        fixtures.push(fixture);
    }

    fixtures
}
