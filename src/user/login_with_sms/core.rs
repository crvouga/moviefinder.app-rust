use crate::{
    core::session::session_id::SessionId,
    ctx::Ctx,
    user::{
        user_account::user_account_::UserAccount, user_profile::user_profile_::UserProfile,
        user_session::user_session_::UserSession,
    },
};

use super::verify_sms::interface;
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

    let sent_code = ctx.verify_sms.send_code(&phone_number).await;

    if let Err(err) = sent_code {
        return Err(SendCodeError::Error(err.to_string()));
    }

    Ok(())
}

pub enum VerifyCodeError {
    InvalidCode(String),
    Error(std::io::Error),
}

pub struct VerifyCodeOk {
    pub account: UserAccount,
    pub profile: UserProfile,
}

pub async fn verify_code(
    ctx: &Ctx,
    session_id: &SessionId,
    phone_number: &str,
    code_input: &str,
) -> Result<VerifyCodeOk, VerifyCodeError> {
    if code_input.is_empty() {
        return Err(VerifyCodeError::InvalidCode("Code is required".to_string()));
    }

    let verified = ctx.verify_sms.verify_code(&phone_number, &code_input).await;

    match verified {
        Err(interface::VerifyCodeError::WrongCode) => {
            Err(VerifyCodeError::InvalidCode("Wrong code".to_string()))
        }

        Err(interface::VerifyCodeError::Error(err)) => Err(VerifyCodeError::Error(err)),

        Ok(()) => {
            let existing_account = ctx
                .user_account_db
                .find_one_by_phone_number(&phone_number)
                .await
                .map_err(VerifyCodeError::Error)?;

            let account_new =
                existing_account.unwrap_or(UserAccount::new(phone_number.to_string()));

            let user_id = account_new.user_id.clone();

            ctx.user_account_db
                .upsert_one(&account_new)
                .await
                .map_err(VerifyCodeError::Error)?;

            let existing_profile = ctx
                .user_profile_db
                .find_one_by_user_id(&user_id)
                .await
                .map_err(VerifyCodeError::Error)?;

            let profile_new = existing_profile.unwrap_or(UserProfile::new(&user_id));

            ctx.user_profile_db
                .upsert_one(&profile_new)
                .await
                .map_err(VerifyCodeError::Error)?;

            let existing_session = ctx
                .user_session_db
                .find_one_by_session_id(&session_id)
                .await
                .map_err(VerifyCodeError::Error)?;

            let session_new = existing_session.unwrap_or(UserSession::new(&user_id, &session_id));

            ctx.user_session_db
                .upsert_one(&session_new)
                .await
                .map_err(VerifyCodeError::Error)?;

            Ok(VerifyCodeOk {
                account: account_new,
                profile: profile_new,
            })
        }
    }
}
