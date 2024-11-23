use std::sync::Arc;

use crate::core::{http::client::HttpClient, logger::interface::Logger};

pub struct Ctx {
    pub http_client: Arc<HttpClient>,
    pub logger: Arc<dyn Logger>,
}

impl Ctx {
    pub async fn new(http_client: Arc<HttpClient>, logger: Arc<dyn Logger>) -> Self {
        Self {
            http_client: http_client.clone(),
            logger: logger.clone(),
        }
    }
}
