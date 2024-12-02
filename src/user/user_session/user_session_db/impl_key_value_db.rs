use crate::{
    core::session::session_id::SessionId,
    key_value_db::interface::KeyValueDb,
    user::{user_id::UserId, user_session::user_session_::UserSession},
};
use async_trait::async_trait;
use std::sync::Arc;

use super::interface::UserSessionDb;

pub struct ImplKeyValueDb {
    session_by_session_id: Box<dyn KeyValueDb>,
    session_id_by_user_id: Box<dyn KeyValueDb>,
}

impl ImplKeyValueDb {
    pub fn new(key_value_db: Arc<dyn KeyValueDb>) -> Self {
        Self {
            session_by_session_id: key_value_db
                .clone()
                .child(vec!["session_by_session_id".to_string()]),

            session_id_by_user_id: key_value_db.child(vec!["session_id_by_user_id".to_string()]),
        }
    }
}

#[async_trait]
impl UserSessionDb for ImplKeyValueDb {
    async fn find_one_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserSession>, std::io::Error> {
        let maybe_session_id = self.session_id_by_user_id.get(user_id.as_str()).await?;

        let session_id = match maybe_session_id {
            Some(id) => SessionId::new(&id).unwrap_or_default(),
            None => return Ok(None),
        };

        self.find_one_by_session_id(&session_id).await
    }

    async fn find_one_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<UserSession>, std::io::Error> {
        let maybe_session = self.session_by_session_id.get(&session_id.as_str()).await?;

        let session = match maybe_session {
            Some(session) => session,
            None => return Ok(None),
        };

        let parsed = serde_json::from_str::<UserSession>(&session)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))?;

        Ok(Some(parsed))
    }

    async fn upsert_one(&self, session: &UserSession) -> Result<(), std::io::Error> {
        let session_id = session.session_id.as_str().to_string();
        let user_id = session.user_id.as_str().to_string();

        let serialized = serde_json::to_string(session)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))?;

        self.session_by_session_id
            .put(&session_id, serialized.to_string())
            .await?;

        self.session_id_by_user_id
            .put(&user_id, session_id.to_string())
            .await?;

        Ok(())
    }
}
