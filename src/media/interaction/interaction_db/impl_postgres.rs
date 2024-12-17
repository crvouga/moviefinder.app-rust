use super::interface::MediaInteractionDb;
use crate::{
    core::{
        db_conn_sql::{self, interface::DbConnSqlDyn},
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
    db_conn_sql: DbConnSqlDyn,
}

impl Postgres {
    pub fn new(db_conn_sql: DbConnSqlDyn) -> Self {
        Self { db_conn_sql }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl Row {
    fn to_media_interaction(self) -> Option<MediaInteraction> {
        let interaction_name =
            InteractionName::from_string(self.interaction_name.unwrap_or_default())?;

        let interaction_action =
            InteractionAction::from_string(self.interaction_action.unwrap_or_default())?;

        Some(MediaInteraction {
            interaction_name,
            interaction_action,
            id: self.id.unwrap_or_default().into(),
            media_id: self.media_id.unwrap_or_default().into(),
            user_id: self.user_id.unwrap_or_default().into(),
            created_at_posix: self.created_at_posix.unwrap_or_default().into(),
        })
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
    async fn find_by_user_id_and_media_ids(
        &self,
        user_id: &UserId,
        media_ids: &Vec<&MediaId>,
    ) -> Result<Vec<MediaInteraction>, std::io::Error> {
        let mut query = Sql::new(
            r#"
            SELECT 
                id,
                media_id,
                user_id,
                interaction_name::TEXT,
                interaction_action::TEXT,
                created_at_posix,
                updated_at_posix 
            FROM 
                media_interaction 
            WHERE 
                    user_id = :user_id 
                AND media_id IN (:media_ids)
            "#,
        );

        query.set(
            "user_id",
            SqlVarType::Primitive(SqlPrimitive::Text(user_id.as_str().to_string())),
        );

        query.set(
            "media_ids",
            SqlVarType::Commas(
                media_ids
                    .iter()
                    .map(|id| SqlPrimitive::Text(id.as_str().to_string()))
                    .collect(),
            ),
        );

        let rows = db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json)
            .await?
            .into_iter()
            .filter_map(|r| r.to_media_interaction())
            .collect::<Vec<MediaInteraction>>();

        Ok(rows)
    }

    async fn find_by_user_id_and_interaction_name(
        &self,
        user_id: &UserId,
        interaction_name: &InteractionName,
    ) -> Result<Vec<MediaInteraction>, std::io::Error> {
        let mut query = Sql::new(
            r#"
            SELECT 
                id,
                media_id,
                user_id,
                interaction_name::TEXT,
                interaction_action::TEXT,
                created_at_posix,
                updated_at_posix 
            FROM 
                media_interaction 
            WHERE 
                    user_id = :user_id 
                AND interaction_name = :interaction_name
            "#,
        );

        query.set(
            "user_id",
            SqlVarType::Primitive(SqlPrimitive::Text(user_id.as_str().to_string())),
        );

        query.set(
            "interaction_name",
            SqlVarType::Primitive(SqlPrimitive::Text(interaction_name.to_postgres_enum())),
        );

        let rows = db_conn_sql::query(self.db_conn_sql.clone(), &query, parse_row_json)
            .await?
            .into_iter()
            .filter_map(|r| r.to_media_interaction())
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
