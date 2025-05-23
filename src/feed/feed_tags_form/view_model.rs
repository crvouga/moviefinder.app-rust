use super::form_state::FormState;
use crate::{
    core::{
        pagination::Paginated,
        query::{Query, QueryFilter, QueryOp},
    },
    ctx::Ctx,
    feed::{
        feed_::Feed, feed_id::FeedId, feed_tag::FeedTag, feed_tag_db::interface::FeedTagQueryField,
    },
};

#[derive(Debug, Clone)]
pub struct ViewModel {
    pub feed: Feed,
    pub tags: Vec<FeedTag>,
    pub form_state: FormState,
    pub search_input: String,
}

impl ViewModel {
    pub fn to_tags(&self) -> Vec<FeedTag> {
        let mut tags = self.tags.clone();
        tags.sort();
        tags.dedup();
        tags
    }

    pub fn to_all_tags(&self) -> Vec<FeedTag> {
        let mut all_tags = self.tags.clone();
        let mut form_state_tags = self.form_state.tags.clone();
        all_tags.append(&mut form_state_tags);
        all_tags.sort();
        all_tags.dedup();
        all_tags
    }

    pub async fn load(ctx: &Ctx, feed_id: &FeedId, search_input: &str) -> Self {
        let feed: Feed = ctx.feed_db.get_else_default(feed_id.clone()).await;
        let form_state = FormState::load(ctx, &feed).await;

        let mut existing_tags: Vec<FeedTag> = form_state
            .tags
            .iter()
            .chain(feed.tags.clone().iter())
            .cloned()
            .collect();

        existing_tags.dedup();

        let feed_tags: Vec<FeedTag> = match search_input {
            "" => {
                let mut feed_tags: Vec<FeedTag> = ctx
                    .feed_tags_db
                    .query(Query {
                        offset: 0,
                        limit: 100,
                        filter: QueryFilter::None,
                    })
                    .await
                    .unwrap_or(Paginated::default())
                    .items
                    .iter()
                    .chain(existing_tags.iter())
                    .cloned()
                    .collect::<Vec<FeedTag>>();

                feed_tags.dedup();
                feed_tags
            }
            _ => {
                let feed_tags = ctx
                    .feed_tags_db
                    .query(Query {
                        offset: 0,
                        limit: 100,
                        filter: QueryFilter::Clause(
                            FeedTagQueryField::Label,
                            QueryOp::Like,
                            search_input.to_string(),
                        ),
                    })
                    .await
                    .unwrap_or(Paginated::default())
                    .items;

                feed_tags
            }
        };

        let feed: Feed = ctx.feed_db.get_else_default(feed_id.clone()).await;

        let form_state = FormState::load(ctx, &feed).await;

        let model = ViewModel {
            feed,
            tags: feed_tags,
            search_input: search_input.to_string(),
            form_state,
        };

        model
    }
}
