use super::form_state::FormState;
use crate::{
    core::logger::interface::Logger, feed::feed_id::FeedId, key_value_db::interface::KeyValueDb,
};
use std::sync::Arc;

mod interface_test;

pub struct FormStateDb {
    key_value_db: Box<dyn KeyValueDb>,
    logger: Arc<dyn Logger>,
}

impl FormStateDb {
    pub fn new(logger: Arc<dyn Logger>, key_value_db: Arc<dyn KeyValueDb>) -> Self {
        Self {
            logger: logger.child("form_state_db"),
            key_value_db: key_value_db.child(vec![
                "feed".to_string(),
                "controls".to_string(),
                "form-state".to_string(),
            ]),
        }
    }

    pub async fn get(&self, feed_id: &FeedId) -> Result<Option<FormState>, String> {
        let got = self.key_value_db.get(feed_id.as_str()).await.unwrap();

        if got.is_none() {
            return Ok(None);
        }

        let parsed = serde_json::from_str(&got.unwrap())
            .map_err(|e| e.to_string())
            .unwrap();

        Ok(Some(parsed))
    }

    pub async fn put(&self, form_state: FormState) -> Result<(), String> {
        let value = serde_json::to_string(&form_state).map_err(|e| e.to_string())?;

        self.key_value_db
            .put(form_state.feed_id.as_str(), value)
            .await
    }
}
