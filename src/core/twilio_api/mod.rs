use super::base64;
use crate::core::http::client::HttpClient;
use std::sync::Arc;

pub mod verify;

pub struct TwilioApi {
    twilio_service_sid: String,
    twilio_auth_token: String,
    twilio_account_sid: String,
    http_client: Arc<HttpClient>,
}

impl TwilioApi {
    pub fn new(
        twilio_service_sid: String,
        twilio_auth_token: String,
        twilio_account_sid: String,
        http_client: Arc<HttpClient>,
    ) -> Self {
        TwilioApi {
            twilio_service_sid,
            twilio_auth_token,
            twilio_account_sid,
            http_client,
        }
    }

    fn to_basic_auth(&self) -> String {
        format!("Basic {}", self.to_basic_auth_value())
    }
    fn to_basic_auth_value(&self) -> String {
        base64::encode(&format!(
            "{}:{}",
            self.twilio_account_sid, self.twilio_auth_token
        ))
    }
}
