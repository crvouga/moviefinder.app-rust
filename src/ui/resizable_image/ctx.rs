use std::sync::Arc;

use crate::core::http::client::HttpClient;

pub struct Ctx {
    pub http_client: Arc<HttpClient>,
}

impl Ctx {
    pub async fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http_client: http_client.clone(),
        }
    }
}
