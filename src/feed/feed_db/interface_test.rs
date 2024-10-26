#[cfg(test)]
mod tests {
    use crate::{
        feed::{
            core::Feed,
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
            feed_db: Box::new(impl_key_value_db::ImplKeyValueDb::new(
                base.ctx.key_value_db,
            )),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let feed = Feed::random();

            let before = f.feed_db.get(feed.feed_id.clone()).await;
            let put = f.feed_db.put(feed.clone()).await;
            let after = f.feed_db.get(feed.feed_id.clone()).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(feed)));
        }
    }
}
