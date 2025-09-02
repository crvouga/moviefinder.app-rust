use super::verify_sms::interface;
use crate::{
    core::{session::session_id::SessionId, unit_of_work::UnitOfWork},
    ctx::Ctx,
    user::{
        user_account::user_account_::UserAccount, user_profile::user_profile_::UserProfile,
        user_session::user_session_::UserSession,
    },
};

#[derive(Debug)]
pub enum SendCodeError {
    InvalidPhoneNumber(String),
    Error(String),
}

impl From<interface::SendCodeError> for SendCodeError {
    fn from(e: interface::SendCodeError) -> Self {
        match e {
            interface::SendCodeError::InvalidPhoneNumber => {
                SendCodeError::InvalidPhoneNumber("Invalid phone number".to_string())
            }
            interface::SendCodeError::Error(err) => SendCodeError::Error(err.to_string()),
        }
    }
}

pub async fn send_code(ctx: &Ctx, phone_number: &str) -> Result<(), SendCodeError> {
    if phone_number.is_empty() {
        return Err(SendCodeError::InvalidPhoneNumber(
            "Phone number is required".to_string(),
        ));
    }

    ctx.user_verify_sms.send_code(&phone_number).await?;

    Ok(())
}

#[derive(Debug)]
pub enum VerifyCodeError {
    InvalidCode(String),
    Error(crate::core::error::CoreError),
}

impl From<interface::VerifyCodeError> for VerifyCodeError {
    fn from(e: interface::VerifyCodeError) -> Self {
        match e {
            interface::VerifyCodeError::WrongCode => {
                VerifyCodeError::InvalidCode("Wrong code".to_string())
            }
            interface::VerifyCodeError::Error(err) => VerifyCodeError::Error(err),
        }
    }
}

impl From<crate::core::error::CoreError> for VerifyCodeError {
    fn from(e: crate::core::error::CoreError) -> Self {
        VerifyCodeError::Error(e)
    }
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

    ctx.user_verify_sms
        .verify_code(&phone_number, &code_input)
        .await?;

    let existing_account = ctx
        .user_account_db
        .find_one_by_phone_number(&phone_number)
        .await
        .map_err(|e| e.namespace("Existing account"))?;

    let account_new = existing_account.unwrap_or(UserAccount::new(phone_number.to_string()));

    let user_id = account_new.user_id.clone();

    let existing_profile = ctx
        .user_profile_db
        .find_one_by_user_id(&user_id)
        .await
        .map_err(|e| e.namespace("Existing profile"))?;

    let profile_new = existing_profile.unwrap_or(UserProfile::new(&user_id));

    let existing_session = ctx
        .user_session_db
        .find_by_session_id(&session_id)
        .await
        .map_err(|e| e.namespace("Existing session"))?;

    let session_new = existing_session.unwrap_or(UserSession::new(&user_id, &session_id));

    transact_user_logged_in(ctx, &account_new, &profile_new, &session_new)
        .await
        .map_err(|e| e.namespace("Transact user logged in"))?;

    Ok(())
}

async fn transact_user_logged_in(
    ctx: &Ctx,
    account: &UserAccount,
    profile: &UserProfile,
    session: &UserSession,
) -> Result<(), crate::core::error::CoreError> {
    UnitOfWork::transact(|uow: UnitOfWork| async move {
        ctx.user_account_db.put(uow.clone(), &account).await?;

        ctx.user_profile_db.put(uow.clone(), &profile).await?;

        ctx.user_session_db.put(uow.clone(), &session).await?;

        Ok(())
    })
    .await?;

    Ok(())
}
