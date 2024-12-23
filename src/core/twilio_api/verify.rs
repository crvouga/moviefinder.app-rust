use serde::{Deserialize, Serialize};

use super::TwilioApi;
use crate::core::http::method::Method;
use crate::core::http::request::Request;
use crate::core::url::query_params::QueryParams;
use crate::core::url::Url;
use crate::core::url_encoded;

pub enum VerifyCodeError {
    WrongCode,
    Error(crate::core::error::Error),
}

pub enum SendCodeError {
    InvalidPhoneNumber,
    Error(crate::core::error::Error),
}

impl TwilioApi {
    pub async fn verify_send_code(&self, phone_number: &str) -> Result<(), SendCodeError> {
        let url = Url {
            host: "verify.twilio.com".to_string(),
            path: format!("/v2/Services/{}/Verifications", self.twilio_service_sid),
            query_params: QueryParams::default(),
        };

        let body = format!("To={}&Channel=sms", url_encoded::encode(phone_number));

        let body = body.into_bytes();

        let headers = vec![
            ("Authorization".to_string(), self.to_basic_auth()),
            (
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
            ("Content-Length".to_string(), body.len().to_string()),
        ]
        .into_iter()
        .collect();

        let request = Request {
            url,
            method: Method::Post,
            headers,
            cookies: Default::default(),
            form_data: Default::default(),
            body,
        };

        let response = self
            .http_client
            .send(request)
            .await
            .map_err(SendCodeError::Error)?;

        if response.status_code <= 299 && response.status_code >= 200 {
            Ok(())
        } else {
            #[derive(Debug, Deserialize, Serialize)]
            struct BodyError {
                code: u16,
            }

            let body_error: BodyError = serde_json::from_slice(&response.body.clone()).unwrap();

            // https://www.twilio.com/docs/api/errors/60200
            if body_error.code == 60200 {
                return Err(SendCodeError::InvalidPhoneNumber);
            }

            Err(SendCodeError::Error(crate::core::error::Error::new(
                format!(
                    "Failed to send code: {:?}",
                    String::from_utf8(response.body).unwrap_or("Unknown error".to_string())
                ),
            )))
        }
    }

    pub async fn verify_verify_code(
        &self,
        phone_number: &str,
        code: &str,
    ) -> Result<(), VerifyCodeError> {
        let url = Url {
            host: "verify.twilio.com".to_string(),
            path: format!("/v2/Services/{}/VerificationCheck", self.twilio_service_sid),
            query_params: QueryParams::default(),
        };

        let body = format!(
            "To={}&Code={}",
            url_encoded::encode(phone_number),
            url_encoded::encode(code)
        )
        .into_bytes();

        let headers = vec![
            ("Authorization".to_string(), self.to_basic_auth()),
            (
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
        ]
        .into_iter()
        .collect();

        let request = Request {
            url,
            method: Method::Post,
            headers,
            cookies: Default::default(),
            form_data: Default::default(),
            body: body.to_vec(),
        };

        let response = self
            .http_client
            .send(request)
            .await
            .map_err(VerifyCodeError::Error)?;

        if response.status_code <= 299 && response.status_code >= 200 {
            #[derive(Debug, Deserialize, Serialize)]
            struct BodyOk {
                valid: bool,
            }

            let body_ok = serde_json::from_slice::<BodyOk>(&response.body).unwrap();

            if body_ok.valid {
                return Ok(());
            }

            return Err(VerifyCodeError::WrongCode);
        } else {
            #[derive(Debug, Deserialize, Serialize)]
            struct BodyError {
                code: u16,
            }

            let body_error: BodyError = serde_json::from_slice(&response.body.clone()).unwrap();

            // https://www.twilio.com/docs/errors/60200
            if body_error.code == 60200 {
                return Err(VerifyCodeError::WrongCode);
            }

            Err(VerifyCodeError::Error(crate::core::error::Error::new(
                format!(
                    "Failed to verify code: {:?}",
                    String::from_utf8(response.body).unwrap_or("Unknown error".to_string())
                ),
            )))
        }
    }
}
