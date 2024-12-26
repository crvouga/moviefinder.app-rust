use std::sync::Arc;

use crate::core::key_value_db::interface::KeyValueDb;

pub struct ImplKeyValueDb {
    source: Arc<dyn KeyValueDb>,
    target: Arc<dyn KeyValueDb>,
}
