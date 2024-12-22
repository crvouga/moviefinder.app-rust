#[cfg(test)]
mod tests {
    use crate::{
        core::unit_of_work::uow,
        fixture::BaseFixture,
        media::interaction::{
            interaction_::MediaInteraction,
            interaction_db::interface::MediaInteractionDb,
            interaction_list::list_db::{impl_postgres, interface::MediaInteractionListDb},
            interaction_name::InteractionName,
        },
        user::user_id::UserId,
    };
    use std::sync::Arc;

    struct Fixture {
        list_db: Arc<dyn MediaInteractionListDb>,
        media_interaction_db: Arc<dyn MediaInteractionDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        if base.env.test_env.is_integration() {
            fixtures.push(Fixture {
                list_db: Arc::new(impl_postgres::ImplPostgres::new(
                    base.ctx.db_conn_sql.clone(),
                )),
                media_interaction_db: base.ctx.media_interaction_db.clone(),
            });
        }

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let user_id = UserId::default();

            let interaction_name = InteractionName::Liked;

            let interactions = vec![
                MediaInteraction::random_add(interaction_name.clone(), user_id.clone()),
                MediaInteraction::random_add(interaction_name.clone(), user_id.clone()),
                MediaInteraction::random_add(interaction_name.clone(), user_id.clone()),
            ];

            let u = uow();

            for i in interactions {
                f.media_interaction_db
                    .put(u.clone(), &i)
                    .await
                    .unwrap_or(());
            }

            let lists = f.list_db.find_by_user_id(user_id.clone()).await.unwrap();

            assert!(lists.len() > 0);
        }
    }
}
