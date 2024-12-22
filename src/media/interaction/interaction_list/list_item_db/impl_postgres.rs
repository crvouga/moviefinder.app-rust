use super::interface::MediaInteractionListItemDb;
use crate::{
    core::{
        db_conn_sql::{self, interface::DbConnSqlDyn},
        pagination::Paginated,
        sql::{Sql, SqlPrimitive, SqlVarType},
    },
    list::{
        list::List,
        list_item::{ListItem, ListItemVariant},
    },
    media::interaction::{
        interaction_::{postgres::MediaInteractionPostgresRow, MediaInteraction},
        interaction_list::list_::MediaInteractionList,
        interaction_name::InteractionName,
    },
    user::user_id::UserId,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct ImplPostgres {
    #[allow(dead_code)]
    db_conn_sql: DbConnSqlDyn,
}

impl ImplPostgres {
    #[allow(dead_code)]
    pub fn new(interaction_db: DbConnSqlDyn) -> Self {
        Self {
            db_conn_sql: interaction_db,
        }
    }
}

#[async_trait]
impl MediaInteractionListItemDb for ImplPostgres {
    async fn find_by_user_id_and_interaction_name(
        &self,
        limit: usize,
        offset: usize,
        user_id: UserId,
        interaction_name: InteractionName,
    ) -> Result<Paginated<ListItem>, std::io::Error> {
        let list = MediaInteractionList {
            user_id: user_id.clone(),
            interaction_name: interaction_name.clone(),
        };
        let mut base_query = Sql::new(
            r#"
            WITH latest_interactions AS (
                SELECT DISTINCT ON (user_id, interaction_name, media_id)
                    id, 
                    media_id, 
                    user_id, 
                    interaction_name, 
                    interaction_action, 
                    created_at_posix             
                FROM media_interaction
                WHERE   interaction_name::TEXT = :interaction_name
                AND     user_id = :user_id
                ORDER BY user_id ASC, interaction_name ASC, media_id ASC, created_at_posix DESC
            )
            SELECT
                mi.id,
                mi.media_id,
                mi.user_id,
                mi.interaction_name::TEXT,
                mi.interaction_action::TEXT,
                mi.created_at_posix,
                mi.updated_at_posix
            FROM media_interaction mi
            JOIN latest_interactions li
                ON mi.user_id = li.user_id
                AND mi.media_id = li.media_id
                AND mi.created_at_posix = li.created_at_posix
                AND mi.interaction_name = li.interaction_name
            WHERE mi.interaction_action = 'add'
            "#,
        );

        base_query.set(
            "interaction_name",
            SqlVarType::Primitive(SqlPrimitive::Text(interaction_name.to_postgres_enum())),
        );

        base_query.set(
            "user_id",
            SqlVarType::Primitive(SqlPrimitive::Text(user_id.as_str().to_string())),
        );

        let total_query = Sql::new(&format!(
            r#"
                SELECT COUNT(*) AS total_count
                FROM ({}) AS subquery
            "#,
            base_query.to_string().as_str()
        ));

        #[derive(Debug, Deserialize, Serialize)]
        struct TotalCount {
            total_count: i64,
        }

        let rows: Vec<TotalCount> =
            db_conn_sql::query(self.db_conn_sql.clone(), &total_query).await?;

        let total = rows
            .first()
            .and_then(|x| Some(x.total_count))
            .unwrap_or_default() as usize;

        let mut query = Sql::new(&format!(
            r#"
            {}
            ORDER BY created_at_posix DESC
            LIMIT :limit
            OFFSET :offset
            "#,
            base_query.to_string().as_str()
        ));

        query.set(
            "limit",
            SqlVarType::Primitive(SqlPrimitive::Number(limit as f64)),
        );

        query.set(
            "offset",
            SqlVarType::Primitive(SqlPrimitive::Number(offset as f64)),
        );

        let rows: Vec<MediaInteractionPostgresRow> =
            db_conn_sql::query(self.db_conn_sql.clone(), &query).await?;

        let rows = rows
            .into_iter()
            .filter_map(|r| r.to_media_interaction())
            .collect::<Vec<MediaInteraction>>();

        let items = rows
            .into_iter()
            .map(|interaction| {
                let list_id = list.id();
                let list_item_id = interaction
                    .interaction_name
                    .to_list_item_id(list_id.clone(), interaction.media_id.clone());

                ListItem {
                    id: list_item_id,
                    list_id,
                    created_at_posix: interaction.created_at_posix,
                    variant: ListItemVariant::Media(interaction.media_id),
                }
            })
            .collect();

        let result = Ok(Paginated {
            total,
            items,
            limit,
            offset,
        });

        result
    }
}
