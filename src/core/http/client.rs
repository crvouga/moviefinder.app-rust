use crate::core::http::response::Response;
use crate::core::logger::interface::Logger;
use crate::core::url::Url;
use crate::debug;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Duration;

use super::request::Request;

pub struct HttpClient {
    simulate_latency: Option<Duration>,
    logger: Arc<dyn Logger>,
}

impl HttpClient {
    pub fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            simulate_latency: None,
            logger: logger.child("http_client"),
        }
    }

    pub fn simulate_latency(mut self, duration: Option<Duration>) -> Self {
        self.simulate_latency = duration;
        self
    }

    pub async fn send(&self, request: Request) -> Result<Response, std::io::Error> {
        debug!(self.logger, "send {:?}", request);

        if let Some(dur) = self.simulate_latency {
            tokio::time::sleep(dur).await;
        }

        let client = Client::new();
        let reqwest_request = request.to_reqwest_request()?;
        let reqwest_response = client.execute(reqwest_request).await.map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to send request: {:?}", e),
            )
        })?;
        let response = Response::from_reqwest_response(reqwest_response).await?;

        Ok(response)
    }
}

impl Request {
    fn to_reqwest_request(self) -> Result<reqwest::Request, std::io::Error> {
        let mut req = reqwest::Request::new(self.method.into(), self.url.into());

        *req.body_mut() = Some(reqwest::Body::from(self.body));

        let headers = req.headers_mut();
        for (key, value) in self.headers {
            let val = HeaderValue::from_str(value.as_str()).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string())
            })?;
            headers.insert(
                HeaderName::from_lowercase(&key.to_lowercase().as_bytes()).unwrap(),
                val,
            );
        }

        Ok(req)
    }
}

impl Into<reqwest::Method> for super::method::Method {
    fn into(self) -> reqwest::Method {
        match self {
            super::method::Method::Get => reqwest::Method::GET,
            super::method::Method::Post => reqwest::Method::POST,
            super::method::Method::Put => reqwest::Method::PUT,
            super::method::Method::Delete => reqwest::Method::DELETE,
            super::method::Method::Patch => reqwest::Method::PATCH,
            super::method::Method::Options => reqwest::Method::OPTIONS,
            super::method::Method::Head => reqwest::Method::HEAD,
            super::method::Method::Connect => reqwest::Method::CONNECT,
            super::method::Method::Trace => reqwest::Method::TRACE,
        }
    }
}

impl Into<reqwest::Url> for Url {
    fn into(self) -> reqwest::Url {
        let url = reqwest::Url::parse(&self.to_string()).unwrap();
        url
    }
}

impl Response {
    pub async fn from_reqwest_response(
        reqwest_response: reqwest::Response,
    ) -> Result<Self, std::io::Error> {
        let status_code = reqwest_response.status().as_u16();
        let headers = reqwest_response
            .headers()
            .iter()
            .map(|(key, value)| {
                (
                    key.as_str().to_string(),
                    value.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect::<HashMap<String, String>>();

        let body = reqwest_response
            .bytes()
            .await
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read response body: {:?}", e),
                )
            })?
            .to_vec();

        Ok(Response {
            status_code,
            headers,
            body,
        })
    }
}
