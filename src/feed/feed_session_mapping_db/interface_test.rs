#[cfg(test)]
mod tests {
    use crate::{
        core::session::session_id::SessionId,
        feed::{
            feed_id::FeedId,
            feed_session_mapping_db::{impl_key_value_db, interface::FeedSessionMappingDb},
        },
        fixture::BaseFixture,
    };

    struct Fixture {
        feed_session_mapping_db: Box<dyn FeedSessionMappingDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            feed_session_mapping_db: Box::new(impl_key_value_db::ImplKeyValueDb::new(
                base.ctx.key_value_db.clone(),
            )),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let feed_id = FeedId::default();
            let session_id = SessionId::default();

            let before = f.feed_session_mapping_db.get(session_id.clone()).await;

            let put = f
                .feed_session_mapping_db
                .put(session_id.clone(), feed_id.clone())
                .await;

            let after = f.feed_session_mapping_db.get(session_id.clone()).await;

            assert_eq!(before.unwrap(), None);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), Some(feed_id));
        }
    }
}
