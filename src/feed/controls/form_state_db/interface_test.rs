#[cfg(test)]
mod tests {

    use crate::{
        feed::{
            controls::{form_state::FormState, form_state_db::FormStateDb},
            feed_::Feed,
        },
        fixture::BaseFixture,
    };

    struct Fixture {
        form_state_db: Box<FormStateDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            form_state_db: Box::new(FormStateDb::new(
                base.ctx.logger.noop(),
                base.ctx.key_value_db,
            )),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let feed = Feed::default();
            let form_state = FormState::new(&feed);

            let feed_id = feed.feed_id.clone();
            let before = f.form_state_db.get(&feed_id).await;
            let put = f.form_state_db.put(&form_state).await;
            let after = f.form_state_db.get(&feed_id).await;

            assert_eq!(before, Ok(None));
            assert_eq!(put, Ok(()));
            assert_eq!(after, Ok(Some(form_state)));
        }
    }
}
