use super::interface::MediaInteractionListDb;
use crate::{
    core::db_conn_sql::interface::DbConnSqlDyn,
    media::interaction::{
        interaction_list::list_::MediaInteractionList, interaction_name::to_all_interaction_names,
    },
    user::user_id::UserId,
};
use async_trait::async_trait;

pub struct ImplPostgres {
    #[allow(dead_code)]
    db_conn_sql: DbConnSqlDyn,
}

impl ImplPostgres {
    #[allow(dead_code)]
    pub fn new(db_conn_sql: DbConnSqlDyn) -> Self {
        Self { db_conn_sql }
    }
}

#[async_trait]
impl MediaInteractionListDb for ImplPostgres {
    async fn find_by_user_id(
        &self,
        user_id: UserId,
    ) -> Result<Vec<MediaInteractionList>, crate::core::error::Error> {
        let lists = to_all_interaction_names()
            .iter()
            .map(|name| MediaInteractionList {
                user_id: user_id.clone(),
                interaction_name: name.clone(),
            })
            .collect();

        Ok(lists)
    }
}
