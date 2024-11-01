use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::core::logger::interface::Logger;

use super::request::HttpRequest;
use super::response::HttpResponse;

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

    pub async fn send(&self, request: HttpRequest) -> tokio::io::Result<HttpResponse> {
        if let Some(dur) = self.simulate_latency {
            tokio::time::sleep(dur).await;
        }

        let addr = format!("{}:80", request.host);
        let mut stream = TcpStream::connect(addr).await?;

        let request_string = request.to_http_string();
        stream.write_all(request_string.as_bytes()).await?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;

        let response_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

        let response = HttpResponse::from_http_string(&response_string);

        Ok(response)
    }
}
