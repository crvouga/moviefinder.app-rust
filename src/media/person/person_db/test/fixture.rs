#[cfg(test)]
use crate::{
    env,
    fixture::BaseFixture,
    media::person::person_db::{impl_tmdb, interface::MediaPersonDb},
};

#[cfg(test)]
pub struct Fixture {
    pub person_db: Box<dyn MediaPersonDb>,
}

#[cfg(test)]
pub async fn fixtures() -> Vec<Fixture> {
    let base_fixture = BaseFixture::new().await;
    let mut fixtures: Vec<Fixture> = vec![];

    if base_fixture.env.test_env == env::TestEnv::Integration {
        let fixture = Fixture {
            person_db: Box::new(impl_tmdb::Tmdb::new(
                base_fixture.ctx.log.clone(),
                base_fixture.ctx.tmdb_api.clone(),
            )),
        };

        fixtures.push(fixture);
    }

    fixtures
}
