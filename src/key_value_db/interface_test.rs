#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use crate::{
        core::uuid,
        env::{Env, TestEnv},
        fixture::BaseFixture,
        key_value_db::{
            impl_hash_map::ImplHashMap, impl_postgres::ImplPostgres, interface::KeyValueDb,
        },
    };

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

        let env = Env::load().unwrap();

        let mut fixtures: Vec<Fixture> = vec![];

        fixtures.push(Fixture {
            key_value_db: Box::new(ImplHashMap::new()),
        });

        if env.test_env == TestEnv::Integration {
            fixtures.push(Fixture {
                key_value_db: Box::new(ImplPostgres::new(
                    base_fixture.ctx.logger,
                    base_fixture.ctx.db_conn_sql,
                )),
            });
        }

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or("".to_string());

            let before = f.key_value_db.get(&item.id).await;
            let put = f.key_value_db.put(&item.id, item_serialized.clone()).await;
            let after = f.key_value_db.get(&item.id).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(item_serialized)));
        }
    }

    #[tokio::test]
    async fn test_updating_existing() {
        for f in fixtures().await {
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or("".to_string());
            let updated_item = Item {
                id: item.id.clone(),
                value: "Updated value".to_string(),
            };
            let updated_item_serialized =
                serde_json::to_string(&updated_item).unwrap_or("".to_string());

            let before = f.key_value_db.get(&item.id).await;
            let put = f.key_value_db.put(&item.id, item_serialized.clone()).await;
            let after = f.key_value_db.get(&item.id).await;

            let before_update = f.key_value_db.get(&updated_item.id).await;
            let put_update = f
                .key_value_db
                .put(&updated_item.id, updated_item_serialized.clone())
                .await;
            let after_update = f.key_value_db.get(&updated_item.id).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(item_serialized.clone())));
            assert_eq!(before_update, Ok(Some(item_serialized)));
            assert_eq!(put_update, Ok(()));
            assert_eq!(after_update, Ok(Some(updated_item_serialized)));
        }
    }

    #[tokio::test]
    async fn test_zap() {
        for f in fixtures().await {
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or("".to_string());

            let before = f.key_value_db.get(&item.id).await;
            let put = f.key_value_db.put(&item.id, item_serialized.clone()).await;
            let after = f.key_value_db.get(&item.id).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(item_serialized.clone())));

            let zap = f.key_value_db.zap(&item.id).await;
            let after_zap = f.key_value_db.get(&item.id).await;

            assert_eq!(zap, Ok(()));
            assert_eq!(after_zap, Ok(None));
        }
    }

    #[tokio::test]
    async fn test_child() {
        for f in fixtures().await {
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or("".to_string());

            let before = f.key_value_db.get(&item.id).await;
            let put = f.key_value_db.put(&item.id, item_serialized.clone()).await;
            let after = f.key_value_db.get(&item.id).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(item_serialized.clone())));

            let child = f.key_value_db.child(vec!["child".to_string()]);
            let child_item = Item::random();
            let child_item_serialized =
                serde_json::to_string(&child_item).unwrap_or("".to_string());

            let before_child = child.get(&child_item.id).await;
            let put_child = child
                .put(&child_item.id, child_item_serialized.clone())
                .await;
            let after_child = child.get(&child_item.id).await;

            assert_eq!(before_child, Ok(None));
            assert_eq!(put_child, Ok(()));
            assert_eq!(after_child, Ok(Some(child_item_serialized)));
        }
    }
}
