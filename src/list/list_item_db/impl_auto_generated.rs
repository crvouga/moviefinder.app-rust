use super::interface::ListItemDb;
use crate::{
    core::{
        pagination::Paginated,
        query::{Query, QueryFilter, QueryOp},
        unit_of_work::UnitOfWork,
    },
    list::{
        core::{list::ListVariant, list_id::ListId, list_item::ListItem},
        list_db::interface::{ListDb, ListQueryField},
    },
    media::interaction::interaction_db::interface::MediaInteractionDb,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImplAutoGenerated {
    media_interaction_db: Arc<dyn MediaInteractionDb>,
    list_db: Arc<dyn ListDb>,
}

impl ImplAutoGenerated {
    #[allow(dead_code)]
    pub fn new(
        media_interaction_db: Arc<dyn MediaInteractionDb>,
        list_db: Arc<dyn ListDb>,
    ) -> Self {
        Self {
            media_interaction_db,
            list_db,
        }
    }
}

#[async_trait]
impl ListItemDb for ImplAutoGenerated {
    async fn find_by_list_id(
        &self,
        limit: usize,
        offset: usize,
        list_id: ListId,
    ) -> Result<Paginated<ListItem>, std::io::Error> {
        let maybe_list = self
            .list_db
            .query(Query {
                limit: limit.clone(),
                offset: offset.clone(),
                filter: QueryFilter::Clause(
                    ListQueryField::ListId,
                    QueryOp::Eq,
                    list_id.as_str().to_string(),
                ),
            })
            .await?
            .items
            .first()
            .map(|v| v.to_owned());

        let empty_response = Ok(Paginated {
            items: vec![],
            total: 0,
            limit,
            offset,
        });

        let (list, interaction_name) = match maybe_list {
            Some(list) => match list.variant.clone() {
                ListVariant::AutoGenerated(interaction_name) => (list, interaction_name),
                _ => {
                    return empty_response;
                }
            },
            None => {
                return empty_response;
            }
        };

        let interactions = self
            .media_interaction_db
            .find_by_user_id_and_interaction_name(&list.creator_user_id, &interaction_name)
            .await?;

        let list_items: Vec<ListItem> = interactions
            .iter()
            .map(|i| ListItem::from((list.id.clone(), i.clone())))
            .collect();

        let total = list_items.len();

        let items = list_items
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect();

        Ok(Paginated {
            total,
            items,
            limit,
            offset,
        })
    }
    async fn put(
        &self,
        _uow: UnitOfWork,
        _list_items: Vec<ListItem>,
    ) -> Result<(), std::io::Error> {
        unimplemented!();
    }
}
