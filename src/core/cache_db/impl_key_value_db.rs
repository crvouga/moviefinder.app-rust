use super::interface::{CacheDb, CacheResult};
use crate::core::{
    error::Error,
    key_value_db::interface::{KeyValueDbDyn, KeyValueDbExt},
    posix::Posix,
    unit_of_work::UnitOfWork,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Entry {
    key: String,
    value: Vec<u8>,
    ttl: Duration,
    inserted_at_posix: Posix,
}

pub struct ImplKeyValueDb {
    namepsace: Vec<String>,
    entries: KeyValueDbDyn,
}

impl ImplKeyValueDb {
    pub fn new(key_value_db: KeyValueDbDyn) -> Self {
        Self {
            namepsace: vec![],
            entries: key_value_db
                .namespace(vec!["cache".to_string(), "entries".to_string()])
                .into(),
        }
    }
}

#[async_trait]
impl CacheDb for ImplKeyValueDb {
    async fn get_bytes(&self, now: Posix, key: &str) -> CacheResult<Vec<u8>> {
        let got = self.entries.get::<Entry>(key).await;

        let maybe_entry = match got {
            Ok(entry) => entry,
            Err(e) => return CacheResult::Err(e),
        };

        let entry = match maybe_entry {
            None => return CacheResult::Missing,
            Some(entry) => entry,
        };

        let lifetime = now.diff(&entry.inserted_at_posix);
        let lifespan = entry.ttl;
        let is_stale = lifetime >= lifespan;

        if is_stale {
            return CacheResult::Stale(entry.value);
        }

        CacheResult::Fresh(entry.value)
    }

    async fn put_bytes(
        &self,
        uow: UnitOfWork,
        ttl: Duration,
        now: Posix,
        key: &str,
        value: &[u8],
    ) -> Result<(), Error> {
        let entry = Entry {
            key: key.to_string(),
            value: value.to_vec(),
            ttl,
            inserted_at_posix: now,
        };

        self.entries.put(uow, key, &entry).await
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error> {
        self.entries.zap(uow, key).await
    }

    fn namespace(&self, namespace: Vec<String>) -> Box<dyn CacheDb> {
        let new_namespace: Vec<String> = self
            .namepsace
            .clone()
            .into_iter()
            .chain(namespace)
            .collect();

        Box::new(ImplKeyValueDb {
            entries: self.entries.namespace(new_namespace.clone()).into(),
            namepsace: new_namespace,
        })
    }
}
