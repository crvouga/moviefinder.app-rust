#[cfg(test)]
mod tests {
    use crate::{
        core::query::{Query, QueryFilter, QueryOp},
        feed::{
            feed_tag::FeedTag,
            feed_tag_db::{
                self,
                interface::{FeedTagDb, FeedTagQueryField},
            },
        },
        fixture::BaseFixture,
    };

    struct Fixture {
        feed_tag_db: Box<dyn FeedTagDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            feed_tag_db: Box::new(feed_tag_db::impl_::Impl_::new(base.ctx.genre_db)),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get() {
        for f in fixtures().await {
            let queried = f
                .feed_tag_db
                .query(Query {
                    filter: QueryFilter::None,
                    limit: 10,
                    offset: 0,
                })
                .await
                .unwrap()
                .items;

            assert!(queried.len() > 0);
        }
    }

    #[tokio::test]
    async fn test_search() {
        for f in fixtures().await {
            let queried = f
                .feed_tag_db
                .query(Query {
                    filter: QueryFilter::Clause(
                        FeedTagQueryField::Label,
                        QueryOp::Like,
                        "Horror".to_string(),
                    ),
                    limit: 10,
                    offset: 0,
                })
                .await
                .unwrap()
                .items;

            assert!(queried.len() > 0);

            let first = queried.into_iter().next().unwrap();

            assert!(matches!(first, FeedTag::Genre(_)));
            // assert!(first.label().contains("Horror"));
        }
    }
}
