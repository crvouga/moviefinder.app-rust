use std::sync::Arc;

use super::interface::{FeedTagDb, FeedTagQueryField};
use crate::{
    core::{
        pagination::Paginated,
        query::{Query, QueryFilter, QueryOp},
    },
    feed::feed_tag::FeedTag,
    media::genre::genre_db::interface::GenreDb,
    media::person::person_db::interface::{PersonDb, PersonQueryField},
};
use async_trait::async_trait;

pub struct Impl_ {
    genre_db: Arc<dyn GenreDb>,
    person_db: Arc<dyn PersonDb>,
}

impl Impl_ {
    pub fn new(genre_db: Arc<dyn GenreDb>, person_db: Arc<dyn PersonDb>) -> Self {
        Self {
            genre_db,
            person_db,
        }
    }
}

#[async_trait]
impl FeedTagDb for Impl_ {
    async fn query(&self, query: Query<FeedTagQueryField>) -> Result<Paginated<FeedTag>, String> {
        let genres = self.genre_db.get_all().await.unwrap_or(vec![]);

        let people = self
            .person_db
            .query(match &query.filter {
                QueryFilter::Clause(FeedTagQueryField::Label, QueryOp::Like, value) => Query {
                    filter: QueryFilter::Clause(
                        PersonQueryField::Name,
                        QueryOp::Like,
                        value.clone(),
                    ),
                    limit: query.limit,
                    offset: query.offset,
                },
                _ => Query {
                    filter: QueryFilter::None,
                    limit: query.limit,
                    offset: query.offset,
                },
            })
            .await
            .unwrap_or_default()
            .items;

        let genre_feed_tags: Vec<FeedTag> = genres
            .iter()
            .map(|genre| FeedTag::Genre(genre.clone()))
            .collect();

        let people_feed_tags: Vec<FeedTag> = people
            .into_iter()
            .map(|person| FeedTag::Person(person))
            .collect();

        let feed_tags = genre_feed_tags
            .into_iter()
            .chain(people_feed_tags.into_iter())
            .collect::<Vec<FeedTag>>();

        let filtered: Vec<FeedTag> = feed_tags
            .into_iter()
            .filter(|feed_tag| match &query.filter {
                QueryFilter::Clause(FeedTagQueryField::Label, QueryOp::Like, value) => feed_tag
                    .label()
                    .to_lowercase()
                    .contains(value.to_lowercase().as_str()),
                _ => true,
            })
            .collect();

        let total = filtered.len();

        let sliced = filtered
            .into_iter()
            .skip(query.offset)
            .take(query.limit)
            .collect();

        Ok(Paginated {
            total: total,
            items: sliced,
            limit: query.limit,
            offset: query.offset,
        })
    }
}
