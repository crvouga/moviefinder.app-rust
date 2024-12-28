use super::interface::{Cache, Cached};
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
    key: Option<String>,
    value: Option<Vec<u8>>,
    max_age: Option<Duration>,
    inserted_at_posix: Option<Posix>,
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
impl Cache for ImplKeyValueDb {
    async fn get_bytes(&self, now: Posix, key: &str) -> Cached<Vec<u8>> {
        let got = self.entries.get::<Entry>(key).await;

        let maybe_entry = match got {
            Ok(entry) => entry,
            Err(e) => return Cached::Err(e),
        };

        let entry = match maybe_entry {
            None => return Cached::Missing,
            Some(entry) => entry,
        };

        let inserted_at_posix = match entry.inserted_at_posix {
            None => return Cached::Missing,
            Some(inserted_at_posix) => inserted_at_posix,
        };

        let lifetime = now.diff(&inserted_at_posix);
        let lifespan = entry.max_age.unwrap_or_default();
        let is_stale = lifetime >= lifespan;

        let value = match entry.value {
            None => return Cached::Missing,
            Some(value) => value,
        };

        if is_stale {
            return Cached::Stale(value);
        }

        Cached::Fresh(value)
    }

    async fn put_bytes(
        &self,
        uow: UnitOfWork,
        max_age: Duration,
        now: Posix,
        key: &str,
        value: &[u8],
    ) -> Result<(), Error> {
        let entry = Entry {
            key: Some(key.to_string()),
            value: Some(value.to_vec()),
            max_age: Some(max_age),
            inserted_at_posix: Some(now),
        };

        self.entries.put(uow, key, &entry).await
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error> {
        self.entries.zap(uow, key).await
    }

    fn namespace(&self, namespace: Vec<String>) -> Box<dyn Cache> {
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
