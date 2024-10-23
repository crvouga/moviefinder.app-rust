#[cfg(test)]
mod tests {
    use crate::{
        env,
        feed::{feed::Feed, feed_db::interface::FeedDb},
    };

    struct Fixture {
        pub feed_db: Box<dyn FeedDb>,
    }

    fn fixtures() -> Vec<Fixture> {
        let env = env::Env::load();

        let fixtures: Vec<Fixture> = vec![];

        env.test_env == env::TestEnv::Integration;

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures() {
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
