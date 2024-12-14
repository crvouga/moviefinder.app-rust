use super::interface::MediaInteractionDb;
use crate::{
    core::{
        db_conn_sql::{self, interface::DbConnSqlRef},
        sql::{Sql, SqlPrimitive, SqlVarType},
        unit_of_work::UnitOfWork,
    },
    media::{
        interaction::{
            interaction_::MediaInteraction, interaction_action::InteractionAction,
            interaction_name::InteractionName,
        },
        media_id::MediaId,
    },
    user::user_id::UserId,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct Postgres {
    db_conn_sql: DbConnSqlRef,
}

impl Postgres {
    pub fn new(db_conn_sql: DbConnSqlRef) -> Self {
        Self { db_conn_sql }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Row {
    id: Option<String>,
    media_id: Option<String>,
    user_id: Option<String>,
    interaction_name: Option<String>,
    interaction_action: Option<String>,
    created_at_posix: Option<i64>,
    updated_at_posix: Option<i64>,
    deleted_at_posix: Option<i64>,
}

impl From<Row> for MediaInteraction {
    fn from(value: Row) -> Self {
        MediaInteraction {
            id: value.id.unwrap_or_default().into(),
            media_id: value.media_id.unwrap_or_default().into(),
            user_id: value.user_id.unwrap_or_default().into(),
            interaction_name: value.interaction_name.unwrap_or_default().into(),
            interaction_action: value.interaction_action.unwrap_or_default().into(),
            created_at_posix: value.created_at_posix.unwrap_or_default().into(),
        }
    }
}

impl InteractionName {
    fn to_postgres_enum(&self) -> String {
        match self {
            InteractionName::Liked => "liked".to_string(),
            InteractionName::Disliked => "disliked".to_string(),
            InteractionName::Interested => "interested".to_string(),
            InteractionName::NotInterested => "not-interested".to_string(),
            InteractionName::Seen => "seen".to_string(),
            InteractionName::NotSeen => "not-seen".to_string(),
        }
    }
}

impl InteractionAction {
    fn to_postgres_enum(&self) -> String {
        match self {
            InteractionAction::Add => "add".to_string(),
            InteractionAction::Retract => "retract".to_string(),
        }
    }
}

fn parse_row_json(value: serde_json::Value) -> Result<Row, std::io::Error> {
    serde_json::from_value(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[async_trait]
impl MediaInteractionDb for Postgres {
    async fn list_for_media(
        &self,
        user_id: &UserId,
        media_id: &MediaId,
    ) -> Result<Vec<MediaInteraction>, std::io::Error> {
        let mut query = Sql::new(
            r#"
            SELECT 
                id,
                media_id,
                user_id,
                interaction_name,
                interaction_action,
                created_at_posix,
                updated_at_posix,
                deleted_at_posix 
            FROM 
                media_interaction 
            WHERE 
                    user_id = :user_id 
                AND media_id = :media_id
            "#,
        );

        query.set(
            "user_id",
            SqlVarType::Primitive(SqlPrimitive::Text(user_id.as_str().to_string())),
        );

        query.set(
            "media_id",
            SqlVarType::Primitive(SqlPrimitive::Text(media_id.as_str().to_string())),
        );

        let rows = db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json)
            .await?
            .into_iter()
            .map(|r| {
                let interaction: MediaInteraction = r.into();
                interaction
            })
            .collect::<Vec<MediaInteraction>>();

        Ok(rows)
    }
    async fn put(
        &self,
        uow: UnitOfWork,
        interaction: &MediaInteraction,
    ) -> Result<(), std::io::Error> {
        let mut query = Sql::new(
            r#"
            INSERT INTO media_interaction (
                id,
                media_id,
                user_id,
                interaction_name,
                interaction_action,
                created_at_posix
            ) VALUES (
                :id,
                :media_id,
                :user_id,
                :interaction_name,
                :interaction_action,
                :created_at_posix
            )
            "#,
        );

        query.set(
            "id",
            SqlVarType::Primitive(SqlPrimitive::Text(interaction.id.as_str().to_string())),
        );

        query.set(
            "media_id",
            SqlVarType::Primitive(SqlPrimitive::Text(
                interaction.media_id.as_str().to_string(),
            )),
        );

        query.set(
            "user_id",
            SqlVarType::Primitive(SqlPrimitive::Text(interaction.user_id.as_str().to_string())),
        );

        query.set(
            "interaction_name",
            SqlVarType::Primitive(SqlPrimitive::Text(
                interaction.interaction_name.to_postgres_enum(),
            )),
        );

        query.set(
            "interaction_action",
            SqlVarType::Primitive(SqlPrimitive::Text(
                interaction.interaction_action.to_postgres_enum(),
            )),
        );

        query.set(
            "created_at_posix",
            SqlVarType::Primitive(SqlPrimitive::Number(
                interaction.created_at_posix.as_i64() as f64
            )),
        );

        db_conn_sql::execute(self.db_conn_sql.clone(), uow, &query).await?;

        Ok(())
    }
}
