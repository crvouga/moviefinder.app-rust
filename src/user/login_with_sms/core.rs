use super::verify_sms::interface;
use crate::{
    core::{session::session_id::SessionId, unit_of_work::UnitOfWork},
    ctx::Ctx,
    user::{
        user_account::user_account_::UserAccount, user_profile::user_profile_::UserProfile,
        user_session::user_session_::UserSession,
    },
};
pub enum SendCodeError {
    InvalidPhoneNumber(String),
    Error(String),
}

pub async fn send_code(ctx: &Ctx, phone_number: &str) -> Result<(), SendCodeError> {
    if phone_number.is_empty() {
        return Err(SendCodeError::InvalidPhoneNumber(
            "Phone number is required".to_string(),
        ));
    }

    ctx.verify_sms
        .send_code(&phone_number)
        .await
        .map_err(|e| match e {
            interface::SendCodeError::InvalidPhoneNumber => {
                SendCodeError::InvalidPhoneNumber("Invalid phone number".to_string())
            }
            interface::SendCodeError::Error(err) => SendCodeError::Error(err.to_string()),
        })?;

    Ok(())
}

pub enum VerifyCodeError {
    InvalidCode(String),
    Error(std::io::Error),
}

pub async fn verify_code(
    ctx: &Ctx,
    session_id: &SessionId,
    phone_number: &str,
    code_input: &str,
) -> Result<(), VerifyCodeError> {
    if code_input.is_empty() {
        return Err(VerifyCodeError::InvalidCode("Code is required".to_string()));
    }

    ctx.verify_sms
        .verify_code(&phone_number, &code_input)
        .await
        .map_err(|e| match e {
            interface::VerifyCodeError::WrongCode => {
                VerifyCodeError::InvalidCode("Wrong code".to_string())
            }
            interface::VerifyCodeError::Error(err) => VerifyCodeError::Error(err),
        })?;

    let existing_account = ctx
        .user_account_db
        .find_one_by_phone_number(&phone_number)
        .await
        .map_err(VerifyCodeError::Error)?;

    let account_new = existing_account.unwrap_or(UserAccount::new(phone_number.to_string()));

    let user_id = account_new.user_id.clone();

    let existing_profile = ctx
        .user_profile_db
        .find_one_by_user_id(&user_id)
        .await
        .map_err(VerifyCodeError::Error)?;

    let profile_new = existing_profile.unwrap_or(UserProfile::new(&user_id));

    let existing_session = ctx
        .user_session_db
        .find_by_session_id(&session_id)
        .await
        .map_err(VerifyCodeError::Error)?;

    let session_new = existing_session.unwrap_or(UserSession::new(&user_id, &session_id));

    transact_new_user(ctx, &account_new, &profile_new, &session_new)
        .await
        .map_err(VerifyCodeError::Error)?;

    Ok(())
}

async fn transact_new_user(
    ctx: &Ctx,
    account: &UserAccount,
    profile: &UserProfile,
    session: &UserSession,
) -> Result<(), std::io::Error> {
    UnitOfWork::transact(|uow: UnitOfWork| async move {
        ctx.user_account_db
            .upsert_one(uow.clone(), &account)
            .await?;

        ctx.user_profile_db
            .upsert_one(uow.clone(), &profile)
            .await?;

        ctx.user_session_db.upsert(uow.clone(), &session).await?;

        Ok(())
    })
    .await?;

    Ok(())
}
