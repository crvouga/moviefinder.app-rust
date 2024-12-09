use super::interface::{VerifyCodeError, VerifySms};
use crate::core::twilio_api::TwilioApi;
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
    async fn send_code(&self, phone_number: &str) -> Result<(), std::io::Error> {
        self.twilio_api.verify_send_code(phone_number).await?;
        Ok(())
    }

    async fn verify_code(&self, phone_number: &str, code: &str) -> Result<(), VerifyCodeError> {
        self.twilio_api
            .verify_verify_code(phone_number, code)
            .await
            .map_err(|err| VerifyCodeError::Error(err))?;

        Ok(())
    }
}
