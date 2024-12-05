#[cfg(test)]
mod tests {
    use crate::{
        core::unit_of_work::UnitOfWork,
        feed::{
            feed_::Feed,
            feed_db::{impl_key_value_db, interface::FeedDb},
        },
        fixture::BaseFixture,
    };

    struct Fixture {
        feed_db: Box<dyn FeedDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            feed_db: Box::new(impl_key_value_db::KeyValueDb::new(base.ctx.key_value_db)),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let feed = Feed::default();

            let uow = UnitOfWork::new();
            let before = f.feed_db.get(feed.feed_id.clone()).await;
            let put = f.feed_db.put(uow, feed.clone()).await;
            let after = f.feed_db.get(feed.feed_id.clone()).await;

            assert_eq!(before.unwrap(), None);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), Some(feed));
        }
    }
}
