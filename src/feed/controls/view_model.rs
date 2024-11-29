use super::{ctx::Ctx, form_state::FormState};
use crate::{
    core::{
        pagination::Paginated,
        query::{Query, QueryFilter, QueryOp},
    },
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
    pub fn to_seperated_tags(&self) -> (Vec<FeedTag>, Vec<FeedTag>) {
        let mut active: Vec<FeedTag> = self
            .tags
            .clone()
            .into_iter()
            .filter(|feed_tag| self.form_state.tags.contains(feed_tag))
            .collect();
        active.sort();
        active.dedup();

        let mut inactive: Vec<FeedTag> = self
            .tags
            .clone()
            .into_iter()
            .filter(|feed_tag| !self.form_state.tags.contains(feed_tag))
            .collect();
        inactive.sort();
        inactive.dedup();

        (active, inactive)
    }

    pub fn to_tags(&self) -> Vec<FeedTag> {
        let mut tags = self.tags.clone();
        tags.sort();
        tags.dedup();
        tags
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
                    .feed_tag_db
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
                    .feed_tag_db
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
