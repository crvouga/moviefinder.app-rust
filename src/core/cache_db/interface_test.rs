#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        core::{
            cache_db::{
                impl_key_value_db::ImplKeyValueDb,
                interface::{Cache, CacheDbExt, Cached},
            },
            posix::Posix,
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
        pub cache_db: Box<dyn Cache>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let base_fixture = BaseFixture::new().await;
        let fixtures: Vec<Fixture> = vec![Fixture {
            cache_db: Box::new(ImplKeyValueDb::new(base_fixture.ctx.key_value_db.clone())),
        }];

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            // Because of the extension trait, we specify the type we expect from get<T>.
            let before = f.cache_db.get::<String>(Posix::now(), &item.id).await;
            assert_eq!(before, Cached::Missing);

            let put = f
                .cache_db
                .put(
                    uow.clone(),
                    Duration::from_secs(100),
                    Posix::now(),
                    &item.id,
                    &item_serialized,
                )
                .await;
            assert!(put.is_ok());

            let after = f.cache_db.get::<String>(Posix::now(), &item.id).await;
            assert_eq!(after, Cached::Fresh(item_serialized));
        }
    }

    #[tokio::test]
    async fn test_zap() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            let put = f
                .cache_db
                .put(
                    uow.clone(),
                    Duration::from_secs(100),
                    Posix::now(),
                    &item.id,
                    &item_serialized,
                )
                .await;
            assert!(put.is_ok());

            let after = f.cache_db.get::<String>(Posix::now(), &item.id).await;
            assert_eq!(after, Cached::Fresh(item_serialized.clone()));

            let zap = f.cache_db.zap(uow.clone(), &item.id).await;
            assert!(zap.is_ok());

            let after_zap = f.cache_db.get::<String>(Posix::now(), &item.id).await;
            assert_eq!(after_zap, Cached::Missing);
        }
    }

    #[tokio::test]
    async fn test_ttl() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let item = Item::random();
            let item_serialized = serde_json::to_string(&item).unwrap_or_default();

            let now = Posix::now();

            let put = f
                .cache_db
                .put(
                    uow.clone(),
                    Duration::from_secs(1),
                    now.clone(),
                    &item.id,
                    &item_serialized,
                )
                .await;
            assert!(put.is_ok());

            let after = f.cache_db.get::<String>(now.clone(), &item.id).await;
            assert_eq!(after, Cached::Fresh(item_serialized.clone()));

            let after_ttl = f
                .cache_db
                .get::<String>(now.future(Duration::from_secs(2)), &item.id)
                .await;
            assert_eq!(after_ttl, Cached::Stale(item_serialized.clone()));
        }
    }
}
