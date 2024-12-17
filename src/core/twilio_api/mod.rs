use super::{base64, http::client::HttpClientDyn};

pub mod verify;

pub struct TwilioApi {
    http_client: HttpClientDyn,
    twilio_service_sid: String,
    twilio_auth_token: String,
    twilio_account_sid: String,
}

impl TwilioApi {
    pub fn new(
        http_client: HttpClientDyn,
        twilio_service_sid: String,
        twilio_auth_token: String,
        twilio_account_sid: String,
    ) -> Self {
        TwilioApi {
            http_client,
            twilio_service_sid,
            twilio_auth_token,
            twilio_account_sid,
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
