#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{
        core::uuid,
        key_value_db::{impl_hash_map::HashMap, interface::KeyValueDb},
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

    fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        fixtures.push(Fixture {
            key_value_db: Box::new(HashMap::new()),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for mut f in fixtures() {
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
        for mut f in fixtures() {
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or("".to_string());

            let before = f.key_value_db.get(&item.id).await;
            let put = f.key_value_db.put(&item.id, item_serialized.clone()).await;
            let after = f.key_value_db.get(&item.id).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(item_serialized.clone())));

            let updated_item = Item {
                id: item.id.clone(),
                value: "Updated value".to_string(),
            };
            let updated_item_serialized =
                serde_json::to_string(&updated_item).unwrap_or("".to_string());

            let before_update = f.key_value_db.get(&updated_item.id).await;
            let put_update = f
                .key_value_db
                .put(&updated_item.id, updated_item_serialized.clone())
                .await;
            let after_update = f.key_value_db.get(&updated_item.id).await;

            assert_eq!(before_update, Ok(Some(item_serialized)));
            assert_eq!(put_update, Ok(()));
            assert_eq!(after_update, Ok(Some(updated_item_serialized)));
        }
    }

    #[tokio::test]
    async fn test_zap() {
        for mut f in fixtures() {
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
}
