#[cfg(test)]
mod tests {
    use crate::{
        core::unit_of_work::uow,
        fixture::BaseFixture,
        media::interaction::{
            interaction_::MediaInteraction,
            interaction_db::{self, interface::MediaInteractionDb},
        },
    };

    struct Fixture {
        interaction_db: Box<dyn MediaInteractionDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            interaction_db: Box::new(interaction_db::impl_postgres::Postgres::new(
                base.ctx.db_conn_sql.clone(),
            )),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_it_should_work() {
        for f in fixtures().await {
            let interaction = MediaInteraction::random();

            let before = f
                .interaction_db
                .list_by_user_media(&interaction.user_id, &vec![&interaction.media_id])
                .await;

            let put = f.interaction_db.put(uow(), &interaction).await;

            let after = f
                .interaction_db
                .list_by_user_media(&interaction.user_id, &vec![&interaction.media_id])
                .await;

            assert_eq!(before.unwrap(), vec![]);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), vec![interaction]);
        }
    }
}
