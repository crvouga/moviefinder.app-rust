use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{sync::Arc, time::Duration};

use crate::core::{error::Error, posix::Posix, unit_of_work::UnitOfWork};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheResult<T> {
    Fresh(T),
    Stale(T),
    Missing,
    Err(Error),
}

#[async_trait]
pub trait CacheDb: Send + Sync {
    async fn get_bytes(&self, now: Posix, key: &str) -> CacheResult<Vec<u8>>;

    async fn put_bytes(
        &self,
        uow: UnitOfWork,
        ttl: Duration,
        now: Posix,
        key: &str,
        value: &[u8],
    ) -> Result<(), Error>;

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error>;

    fn namespace(&self, namespace: Vec<String>) -> Box<dyn CacheDb>;
}

#[async_trait]
pub trait CacheDbExt: CacheDb {
    async fn get_now<T>(&self, key: &str) -> CacheResult<T>
    where
        T: DeserializeOwned + Send,
    {
        self.get(Posix::now(), key).await
    }

    async fn put_now<T>(
        &self,
        uow: UnitOfWork,
        ttl: Duration,
        key: &str,
        value: T,
    ) -> Result<(), Error>
    where
        T: Serialize + Send + Sync,
    {
        self.put(uow, ttl, Posix::now(), key, value).await
    }

    async fn get<T>(&self, now: Posix, key: &str) -> CacheResult<T>
    where
        T: DeserializeOwned + Send,
    {
        let got = self.get_bytes(now, key).await;

        match got {
            CacheResult::Err(e) => CacheResult::Err(e),
            CacheResult::Missing => CacheResult::Missing,
            CacheResult::Stale(bytes) => {
                let parsed = serde_json::from_slice(&bytes).map_err(|e| Error::new(e.to_string()));
                match parsed {
                    Ok(value) => CacheResult::Stale(value),
                    Err(e) => CacheResult::Err(e),
                }
            }
            CacheResult::Fresh(bytes) => {
                let parsed = serde_json::from_slice(&bytes).map_err(|e| Error::new(e.to_string()));
                match parsed {
                    Ok(value) => CacheResult::Fresh(value),
                    Err(e) => CacheResult::Err(e),
                }
            }
        }
    }

    async fn put<T>(
        &self,
        uow: UnitOfWork,
        ttl: Duration,
        now: Posix,
        key: &str,
        value: T,
    ) -> Result<(), Error>
    where
        T: Serialize + Send + Sync,
    {
        let bytes = serde_json::to_vec(&value).map_err(|e| Error::new(e.to_string()))?;
        self.put_bytes(uow, ttl, now, key, &bytes)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}

#[async_trait]
impl<T: CacheDb + ?Sized> CacheDbExt for T {}

pub type CacheDbDyn = Arc<dyn CacheDb>;
