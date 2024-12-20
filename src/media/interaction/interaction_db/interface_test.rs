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

        if base.env.test_env.is_integration() {
            fixtures.push(Fixture {
                interaction_db: Box::new(interaction_db::impl_postgres::Postgres::new(
                    base.ctx.db_conn_sql.clone(),
                )),
            });
        }

        fixtures
    }

    #[tokio::test]
    async fn test_it_should_work() {
        for f in fixtures().await {
            let interaction = MediaInteraction::random();

            let before = f
                .interaction_db
                .find_by_user_id_and_media_ids(&interaction.user_id, &vec![&interaction.media_id])
                .await;

            let put = f.interaction_db.put(uow(), &interaction).await;

            let after = f
                .interaction_db
                .find_by_user_id_and_media_ids(&interaction.user_id, &vec![&interaction.media_id])
                .await;

            assert_eq!(before.unwrap(), vec![]);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), vec![interaction]);
        }
    }

    #[tokio::test]
    async fn test_find_by_interaction_name() {
        for f in fixtures().await {
            let expected = MediaInteraction::random();

            let before = f
                .interaction_db
                .find_by_user_id_and_interaction_name(&expected.user_id, &expected.interaction_name)
                .await
                .unwrap();

            let u = uow();

            f.interaction_db.put(u.clone(), &expected).await.unwrap();

            for _ in 0..3 {
                f.interaction_db
                    .put(u.clone(), &MediaInteraction::random())
                    .await
                    .unwrap();
            }

            let after = f
                .interaction_db
                .find_by_user_id_and_interaction_name(&expected.user_id, &expected.interaction_name)
                .await
                .unwrap();

            assert!(before.iter().all(|i| i.id != expected.id));
            assert!(before
                .iter()
                .all(|i| i.interaction_name == expected.interaction_name),);
            assert!(after.iter().any(|i| i.id == expected.id));
            assert!(after
                .iter()
                .all(|i| i.interaction_name == expected.interaction_name),);
        }
    }
}
