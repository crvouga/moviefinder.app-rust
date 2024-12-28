use super::interface::UserSessionDb;
use crate::{
    core::{
        key_value_db::interface::{KeyValueDbDyn, KeyValueDbExt},
        session::session_id::SessionId,
        unit_of_work::UnitOfWork,
    },
    user::user_session::user_session_::UserSession,
};
use async_trait::async_trait;

pub struct KeyValueDb {
    session_by_session_id: KeyValueDbDyn,
    session_id_by_user_id: KeyValueDbDyn,
}

impl KeyValueDb {
    pub fn new(key_value_db: KeyValueDbDyn) -> Self {
        Self {
            session_by_session_id: key_value_db
                .clone()
                .namespace(vec!["session_by_session_id".to_string()])
                .into(),

            session_id_by_user_id: key_value_db
                .namespace(vec!["session_id_by_user_id".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl UserSessionDb for KeyValueDb {
    // async fn find_by_user_id(
    //     &self,
    //     user_id: &UserId,
    // ) -> Result<Option<UserSession>, crate::core::error::Error> {
    //     let maybe_session_id = self.session_id_by_user_id.get(user_id.as_str()).await?;

    //     let session_id = match maybe_session_id {
    //         Some(id) => SessionId::new(&id).unwrap_or_default(),
    //         None => return Ok(None),
    //     };

    //     self.find_by_session_id(&session_id).await
    // }

    async fn find_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<Option<UserSession>, crate::core::error::Error> {
        self.session_by_session_id
            .get::<UserSession>(session_id.as_str())
            .await
    }

    async fn put(
        &self,
        uow: UnitOfWork,
        session: &UserSession,
    ) -> Result<(), crate::core::error::Error> {
        let session_id = session.session_id.as_str().to_string();
        let user_id = session.user_id.as_str().to_string();

        self.session_by_session_id
            .put(uow.clone(), &session_id, &session)
            .await?;

        self.session_id_by_user_id
            .put(uow.clone(), &user_id, &session.session_id)
            .await?;

        Ok(())
    }

    async fn zap(
        &self,
        uow: UnitOfWork,
        session_id: &SessionId,
    ) -> Result<(), crate::core::error::Error> {
        let session = self.find_by_session_id(session_id).await?;

        if let Some(session) = session {
            let user_id = session.user_id.as_str().to_string();
            self.session_by_session_id
                .zap(uow.clone(), &session_id.as_str())
                .await?;

            self.session_id_by_user_id
                .zap(uow.clone(), &user_id)
                .await?;
        }

        Ok(())
    }
}
