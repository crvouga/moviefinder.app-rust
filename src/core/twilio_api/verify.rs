use super::TwilioApi;
use crate::core::http::method::Method;
use crate::core::http::request::Request;
use crate::core::url::query_params::QueryParams;
use crate::core::url::Url;
use crate::core::url_encoded;

impl TwilioApi {
    pub async fn verify_send_code(&self, phone_number: &str) -> Result<(), std::io::Error> {
        let url = Url {
            host: "https://verify.twilio.com".to_string(),
            path: format!("/v2/Services/{}/Verifications", self.twilio_service_sid),
            query_params: QueryParams::default(),
        };

        let body = format!("To={}&Channel=sms", url_encoded::encode(phone_number));
        println!("body: {:?}", body);
        let body = body.into_bytes();

        let headers = vec![
            ("Authorization".to_string(), self.to_basic_auth()),
            (
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            ),
        ]
        .into_iter()
        .collect();

        println!("to_basic_auth: {:?}", self.to_basic_auth());

        let request = Request {
            url,
            method: Method::Post,
            headers,
            cookies: Default::default(),
            form_data: Default::default(),
            body,
        };

        let response = self.http_client.send(request).await?;

        println!("response: {:?}", String::from_utf8(response.body.clone()));

        if response.status_code == 200 {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to send code: {:?}",
                    String::from_utf8(response.body).unwrap_or("Unknown error".to_string())
                ),
            ))
        }
    }

    pub async fn verify_verify_code(
        &self,
        phone_number: &str,
        code: &str,
    ) -> Result<(), std::io::Error> {
        let url = Url {
            host: "verify.twilio.com".to_string(),
            path: format!("/v2/Services/{}/VerificationCheck", self.twilio_service_sid),
            query_params: QueryParams::default(),
        };

        let body = url_encoded::encode(&format!("To={}&Code={}", phone_number, code)).into_bytes();

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
            body,
        };

        let response = self.http_client.send(request).await?;
        if response.status_code == 200 {
            let search = ("\"valid\":true").to_string();
            let valid = String::from_utf8(response.body)
                .unwrap_or("".to_string())
                .contains(&search);
            if valid {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Code verification failed",
                ))
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Failed to verify code: {:?}",
                    String::from_utf8(response.body).unwrap_or("Unknown error".to_string())
                ),
            ))
        }
    }
}
