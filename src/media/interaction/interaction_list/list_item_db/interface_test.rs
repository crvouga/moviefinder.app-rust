#[cfg(test)]
mod tests {
    use crate::{
        core::{pagination::Pagination, unit_of_work::uow},
        fixture::BaseFixture,
        media::interaction::{
            interaction_::MediaInteraction,
            interaction_action::InteractionAction,
            interaction_db::interface::MediaInteractionDb,
            interaction_id::MediaInteractionId,
            interaction_list::list_item_db::{
                impl_postgres, interface::MediaInteractionListItemDb,
            },
            interaction_name::InteractionName,
        },
        user::user_id::UserId,
    };
    use std::{sync::Arc, time::Duration};

    struct Fixture {
        list_item_db: Arc<dyn MediaInteractionListItemDb>,
        media_interaction_db: Arc<dyn MediaInteractionDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        if base.env.test_env.is_integration() {
            fixtures.push(Fixture {
                list_item_db: Arc::new(impl_postgres::ImplPostgres::new(
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
                f.media_interaction_db.put(u.clone(), &i).await.unwrap();
            }

            let list_items = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 10,
                        offset: 0,
                    },
                    user_id.clone(),
                    interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items.items.len(), 3);
            assert_eq!(list_items.total, 3);
        }
    }

    #[tokio::test]
    async fn test_not_including_retracted_interactions() {
        for f in fixtures().await {
            let first_interaction =
                MediaInteraction::random_add(InteractionName::Liked, UserId::default());

            let second_interaction = MediaInteraction {
                created_at_posix: first_interaction
                    .created_at_posix
                    .future(Duration::from_secs(1)),
                interaction_action: InteractionAction::Retract,
                id: MediaInteractionId::default(),
                ..first_interaction.clone()
            };

            let u = uow();

            let interactions = vec![first_interaction.clone(), second_interaction.clone()];
            for i in interactions {
                f.media_interaction_db.put(u.clone(), &i).await.unwrap();
            }

            let list_items = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 10,
                        offset: 0,
                    },
                    first_interaction.user_id.clone(),
                    first_interaction.interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items.items.len(), 0);
            assert_eq!(list_items.total, 0);
        }
    }

    #[tokio::test]
    async fn test_add_then_retract_then_add_again() {
        for f in fixtures().await {
            let first_interaction =
                MediaInteraction::random_add(InteractionName::Liked, UserId::default());

            let second_interaction = MediaInteraction {
                created_at_posix: first_interaction
                    .created_at_posix
                    .future(Duration::from_secs(1)),
                interaction_action: InteractionAction::Retract,
                id: MediaInteractionId::default(),
                ..first_interaction.clone()
            };

            let third_interaction = MediaInteraction {
                created_at_posix: first_interaction
                    .created_at_posix
                    .future(Duration::from_secs(2)),
                interaction_action: InteractionAction::Add,
                id: MediaInteractionId::default(),
                ..first_interaction.clone()
            };

            let u = uow();

            let interactions = vec![
                first_interaction.clone(),
                second_interaction.clone(),
                third_interaction.clone(),
            ];
            for i in interactions {
                f.media_interaction_db.put(u.clone(), &i).await.unwrap();
            }

            let list_items = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 10,
                        offset: 0,
                    },
                    first_interaction.user_id.clone(),
                    first_interaction.interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items.items.len(), 1);
            assert_eq!(list_items.total, 1);
        }
    }

    #[tokio::test]
    async fn test_pagination() {
        for f in fixtures().await {
            let user_id = UserId::default();
            let interaction_name = InteractionName::Liked;

            let interactions: Vec<MediaInteraction> = (0..15)
                .map(|_| MediaInteraction::random_add(interaction_name.clone(), user_id.clone()))
                .collect();

            let u = uow();

            // Add interactions to the database
            for i in interactions {
                f.media_interaction_db.put(u.clone(), &i).await.unwrap();
            }

            // Test pagination with limit 5 and offset 0
            let list_items_page_1 = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 5,
                        offset: 0,
                    },
                    user_id.clone(),
                    interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items_page_1.items.len(), 5);
            assert_eq!(list_items_page_1.total, 15);

            // Test pagination with limit 5 and offset 5
            let list_items_page_2 = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 5,
                        offset: 5,
                    },
                    user_id.clone(),
                    interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items_page_2.items.len(), 5);
            assert_eq!(list_items_page_2.total, 15);

            // Test pagination with limit 5 and offset 10
            let list_items_page_3 = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 5,
                        offset: 10,
                    },
                    user_id.clone(),
                    interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items_page_3.items.len(), 5);
            assert_eq!(list_items_page_3.total, 15);

            // Test pagination with limit 5 and offset 15 (no items left)
            let list_items_page_4 = f
                .list_item_db
                .find_by_user_id_and_interaction_name(
                    Pagination {
                        limit: 5,
                        offset: 15,
                    },
                    user_id.clone(),
                    interaction_name.clone(),
                )
                .await
                .unwrap();

            assert_eq!(list_items_page_4.items.len(), 0);
            assert_eq!(list_items_page_4.total, 15);
        }
    }
}
