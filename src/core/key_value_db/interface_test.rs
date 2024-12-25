#[cfg(test)]
mod tests {
    use crate::{
        core::{
            key_value_db::{
                impl_cached_postgres::CachedPostgres,
                impl_hash_map::HashMap,
                impl_postgres::Postgres,
                interface::{KeyValueDb, KeyValueDbExt}, // <-- Import the extension trait
            },
            unit_of_work::UnitOfWork,
            uuid,
        },
        fixture::BaseFixture,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Item {
        id: String,
        value: String,
    }

    impl Item {
        fn random() -> Self {
            Self {
                id: uuid::v4(),
                value: "Some value".to_string(),
            }
        }
    }

    struct Fixture {
        pub key_value_db: Box<dyn KeyValueDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let base_fixture = BaseFixture::new().await;
        let mut fixtures: Vec<Fixture> = vec![];

        // Always include the HashMap-based DB
        fixtures.push(Fixture {
            key_value_db: Box::new(HashMap::new()),
        });

        // Include Postgres-based DBs in integration tests
        if base_fixture.env.test_env.is_integration() {
            fixtures.push(Fixture {
                key_value_db: Box::new(Postgres::new(
                    base_fixture.ctx.log.clone(),
                    base_fixture.ctx.db_conn_sql.clone(),
                )),
            });

            fixtures.push(Fixture {
                key_value_db: Box::new(CachedPostgres::new(
                    base_fixture.ctx.log.clone(),
                    base_fixture.ctx.db_conn_sql.clone(),
                )),
            });
        }

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            // Because of the extension trait, we specify the type we expect from get<T>.
            let before = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(before.unwrap(), None);

            let put = f
                .key_value_db
                .put(uow.clone(), &item.id, &item_serialized)
                .await;
            assert!(put.is_ok());

            let after = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(after.unwrap(), Some(item_serialized));
        }
    }

    #[tokio::test]
    async fn test_updating_existing() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            let updated_item = Item {
                id: item.id.clone(),
                value: "Updated value".to_string(),
            };
            let updated_item_serialized = serde_json::to_string(&updated_item).unwrap_or_default();

            let before = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(before.unwrap(), None);

            let put = f
                .key_value_db
                .put(uow.clone(), &item.id, &item_serialized)
                .await;
            assert!(put.is_ok());

            let after = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(after.unwrap(), Some(item_serialized.clone()));

            let before_update = f.key_value_db.get::<String>(&updated_item.id).await;
            // Because it's the same ID, expect the old data here
            assert_eq!(before_update.unwrap(), Some(item_serialized));

            let put_update = f
                .key_value_db
                .put(uow.clone(), &updated_item.id, &updated_item_serialized)
                .await;
            assert!(put_update.is_ok());

            let after_update = f.key_value_db.get::<String>(&updated_item.id).await;
            assert_eq!(after_update.unwrap(), Some(updated_item_serialized));
        }
    }

    #[tokio::test]
    async fn test_zap() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            let before = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(before.unwrap(), None);

            let put = f
                .key_value_db
                .put(uow.clone(), &item.id, &item_serialized)
                .await;
            assert!(put.is_ok());

            let after = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(after.unwrap(), Some(item_serialized.clone()));

            let zap = f.key_value_db.zap(uow.clone(), &item.id).await;
            assert!(zap.is_ok());

            let after_zap = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(after_zap.unwrap(), None);
        }
    }

    #[tokio::test]
    async fn test_child() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            let before = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(before.unwrap(), None);

            let put = f
                .key_value_db
                .put(uow.clone(), &item.id, &item_serialized)
                .await;
            assert!(put.is_ok());

            let after = f.key_value_db.get::<String>(&item.id).await;
            assert_eq!(after.unwrap(), Some(item_serialized.clone()));

            let child = f.key_value_db.child(vec!["child".to_string()]);
            let child_item = Item::random();
            let child_item_serialized = serde_json::to_string(&child_item).unwrap_or_default();

            let before_child = child.get::<String>(&child_item.id).await;
            assert_eq!(before_child.unwrap(), None);

            let put_child = child
                .put(uow.clone(), &child_item.id, &child_item_serialized)
                .await;
            assert!(put_child.is_ok());

            let after_child = child.get::<String>(&child_item.id).await;
            assert_eq!(after_child.unwrap(), Some(child_item_serialized));
        }
    }
}
