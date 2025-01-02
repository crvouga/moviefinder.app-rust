use super::interface::{SendCodeError, VerifyCodeError, VerifySms};
use crate::core::twilio_api::{self, TwilioApi};
use async_trait::async_trait;
use std::sync::Arc;

pub struct Twilio {
    twilio_api: Arc<TwilioApi>,
}

impl Twilio {
    pub fn new(twilio_api: Arc<TwilioApi>) -> Self {
        Twilio { twilio_api }
    }
}

#[async_trait]
impl VerifySms for Twilio {
    async fn send_code(&self, phone_number: &str) -> Result<(), SendCodeError> {
        self.twilio_api.verify_send_code(phone_number).await?;
        Ok(())
    }

    async fn verify_code(&self, phone_number: &str, code: &str) -> Result<(), VerifyCodeError> {
        self.twilio_api
            .verify_verify_code(phone_number, code)
            .await?;

        Ok(())
    }
}

impl From<twilio_api::verify::SendCodeError> for SendCodeError {
    fn from(err: twilio_api::verify::SendCodeError) -> Self {
        match err {
            twilio_api::verify::SendCodeError::InvalidPhoneNumber => {
                SendCodeError::InvalidPhoneNumber
            }
            twilio_api::verify::SendCodeError::Error(err) => SendCodeError::Error(err),
        }
    }
}

impl From<twilio_api::verify::VerifyCodeError> for VerifyCodeError {
    fn from(err: twilio_api::verify::VerifyCodeError) -> Self {
        match err {
            twilio_api::verify::VerifyCodeError::WrongCode => VerifyCodeError::WrongCode,
            twilio_api::verify::VerifyCodeError::Error(err) => VerifyCodeError::Error(err),
        }
    }
}
