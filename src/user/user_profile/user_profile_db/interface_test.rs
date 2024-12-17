#[cfg(test)]
mod tests {
    use crate::{
        core::unit_of_work::UnitOfWork,
        fixture::BaseFixture,
        user::{
            user_id::UserId,
            user_profile::{
                user_profile_::UserProfile,
                user_profile_db::{impl_key_value_db, interface::UserProfileDb},
            },
        },
    };

    struct Fixture {
        user_profile_db: Box<dyn UserProfileDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        let base = BaseFixture::new().await;

        fixtures.push(Fixture {
            user_profile_db: Box::new(impl_key_value_db::KeyValueDb::new(base.ctx.key_value_db)),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_and_put() {
        for f in fixtures().await {
            let profile = UserProfile::new(&UserId::default());

            let uow = UnitOfWork::new();
            let before = f
                .user_profile_db
                .find_one_by_user_id(&profile.user_id)
                .await;
            let put = f.user_profile_db.put(uow, &profile).await;
            let after = f
                .user_profile_db
                .find_one_by_user_id(&profile.user_id)
                .await;

            assert_eq!(before.unwrap(), None);
            assert_eq!(put.unwrap(), ());
            assert_eq!(after.unwrap(), Some(profile));
        }
    }
}
