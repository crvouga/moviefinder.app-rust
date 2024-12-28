#[cfg(test)]
use crate::{
    env,
    fixture::BaseFixture,
    media::{
        genre::{genre_db::interface::MediaGenreDb, genre_id::GenreId},
        media_db::{impl_tmdb, interface::MediaDb},
    },
};
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
pub struct Fixture {
    pub media_db: Box<dyn MediaDb>,
    pub genre_db: Arc<dyn MediaGenreDb>,
}

#[cfg(test)]
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

#[cfg(test)]
pub async fn fixtures() -> Vec<Fixture> {
    let base_fixture = BaseFixture::new().await;
    let mut fixtures: Vec<Fixture> = vec![];

    if base_fixture.env.test_env == env::TestEnv::Integration {
        let fixture = Fixture {
            media_db: Box::new(impl_tmdb::Tmdb::new(
                base_fixture.ctx.log.clone(),
                base_fixture.ctx.tmdb_api.clone(),
                base_fixture.ctx.cache_db.clone(),
            )),
            genre_db: base_fixture.ctx.media_genre_db,
        };

        fixtures.push(fixture);
    }

    fixtures
}
