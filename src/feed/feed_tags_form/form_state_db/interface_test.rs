#[cfg(test)]
mod tests {

    use crate::{
        core::unit_of_work::UnitOfWork,
        feed::{
            feed_::Feed,
            feed_tags_form::{form_state::FormState, form_state_db::FeedTagsFormStateDb},
        },
        fixture::BaseFixture,
    };

    struct Fixture {
        form_state_db: Box<FeedTagsFormStateDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            form_state_db: Box::new(FeedTagsFormStateDb::new(
                base.ctx.log.noop(),
                base.ctx.key_value_db,
            )),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let uow = UnitOfWork::new();
            let feed = Feed::default();
            let form_state = FormState::new(&feed);

            let feed_id = feed.feed_id.clone();
            let before = f.form_state_db.get(&feed_id).await;
            let put = f.form_state_db.put(uow.clone(), &form_state).await;
            let after = f.form_state_db.get(&feed_id).await;

            assert_eq!(before.unwrap(), None);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), Some(form_state));
        }
    }
}
