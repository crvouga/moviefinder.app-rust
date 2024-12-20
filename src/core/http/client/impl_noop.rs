use super::HttpClient;
use crate::core::http::request::Request;
use crate::core::http::response::Response;
use async_trait::async_trait;

pub struct ImplNoop {}

impl ImplNoop {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl HttpClient for ImplNoop {
    async fn send(&self, _request: Request) -> Result<Response, std::io::Error> {
        Ok(Response::new(200))
    }
}
