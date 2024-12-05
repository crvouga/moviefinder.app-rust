use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::core::logger::interface::Logger;
use crate::debug;

use super::request::Request;
use super::response::Response;

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

    pub async fn send(&self, request: Request) -> tokio::io::Result<Response> {
        debug!(self.logger, "send {:?}", request);

        if let Some(dur) = self.simulate_latency {
            tokio::time::sleep(dur).await;
        }

        let addr = format!("{}:80", request.url.host);
        let mut stream = TcpStream::connect(addr).await?;

        let request_string = request.to_http_string();
        stream.write_all(request_string.as_bytes()).await?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;

        let response_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

        let response = Response::from_http_string(&response_string);

        Ok(response)
    }
}
