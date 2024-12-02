#[cfg(test)]
mod tests {
    use crate::{
        core::{session::session_id::SessionId, unit_of_work::UnitOfWork},
        fixture::BaseFixture,
        user::{
            user_id::UserId,
            user_session::{
                user_session_::UserSession,
                user_session_db::{impl_key_value_db, interface::UserSessionDb},
            },
        },
    };

    struct Fixture {
        user_session_db: Box<dyn UserSessionDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            user_session_db: Box::new(impl_key_value_db::ImplKeyValueDb::new(
                base.ctx.key_value_db,
            )),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let session = UserSession::new(&UserId::default(), &SessionId::default());

            let uow = UnitOfWork::new();
            let before = f
                .user_session_db
                .find_one_by_session_id(&session.session_id)
                .await;
            let put = f.user_session_db.upsert_one(uow, &session).await;
            let after = f
                .user_session_db
                .find_one_by_session_id(&session.session_id)
                .await;

            assert_eq!(before.unwrap(), None);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), Some(session));
        }
    }
}
