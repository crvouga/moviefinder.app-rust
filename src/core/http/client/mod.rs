use std::sync::Arc;

use crate::core::http::request::Request;
use crate::core::http::response::Response;
use async_trait::async_trait;

pub mod impl_noop;
pub mod impl_reqwest;

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn send(&self, request: Request) -> Result<Response, crate::core::error::CoreError>;
}

pub type HttpClientDyn = Arc<dyn HttpClient>;
