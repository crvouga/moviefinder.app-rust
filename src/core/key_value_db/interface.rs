use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

use crate::core::{error::Error, unit_of_work::UnitOfWork};

#[async_trait]
pub trait KeyValueDb: Send + Sync {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Error>;
    async fn put_bytes(&self, uow: UnitOfWork, key: &str, value: &[u8]) -> Result<(), Error>;
    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error>;
    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb>;
}

#[async_trait]
pub trait KeyValueDbExt: KeyValueDb {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, Error>
    where
        T: DeserializeOwned + Send,
    {
        match self.get_bytes(key).await? {
            Some(bytes) => Ok(Some(
                serde_json::from_slice(&bytes).map_err(|e| Error::new(e.to_string()))?,
            )),
            None => Ok(None),
        }
    }

    async fn put<T>(&self, uow: UnitOfWork, key: &str, value: &T) -> Result<(), Error>
    where
        T: Serialize + Send + Sync,
    {
        let bytes = serde_json::to_vec(value).map_err(|e| Error::new(e.to_string()))?;
        self.put_bytes(uow, key, &bytes)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}

impl<T: KeyValueDb + ?Sized> KeyValueDbExt for T {}

const SEPARATOR: &str = ":";

pub fn to_namespaced_key(namespace: &[String], key: &str) -> String {
    namespace.join(SEPARATOR) + SEPARATOR + key
}

pub type KeyValueDbDyn = Arc<dyn KeyValueDb>;
