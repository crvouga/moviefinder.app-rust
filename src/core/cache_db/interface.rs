use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, sync::Arc, time::Duration};

use crate::core::{error::Error, posix::Posix, unit_of_work::UnitOfWork};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cached<T> {
    Fresh(T),
    Stale(T),
    Missing,
    Err(Error),
}

impl<T> Cached<T> {
    pub fn to_result(self) -> Result<T, Error> {
        match self {
            Cached::Fresh(value) => Ok(value),
            Cached::Stale(value) => Ok(value),
            Cached::Missing => Err(Error::new("Cache miss")),
            Cached::Err(e) => Err(e),
        }
    }
}

impl<T> From<Error> for Cached<T> {
    fn from(error: Error) -> Self {
        Cached::Err(error)
    }
}

#[async_trait]
pub trait Cache: Send + Sync {
    async fn get_bytes(&self, now: Posix, key: &str) -> Cached<Vec<u8>>;

    async fn put_bytes(
        &self,
        uow: UnitOfWork,
        max_age: Duration,
        now: Posix,
        key: &str,
        value: &[u8],
    ) -> Result<(), Error>;

    #[allow(dead_code)]
    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error>;

    fn namespace(&self, namespace: Vec<String>) -> Box<dyn Cache>;
}

#[async_trait]
pub trait CacheDbExt: Cache {
    async fn get_now<T>(&self, key: &str) -> Cached<T>
    where
        T: DeserializeOwned + Send + Debug,
    {
        let got = self.get(Posix::now(), key).await;
        println!("get: {}\ngot: {:?}", key, got);
        got
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
        let put = self.put(uow, ttl, Posix::now(), key, value).await;
        println!("put: {}\nput: {:?}", key, put);
        put
    }

    async fn get<T>(&self, now: Posix, key: &str) -> Cached<T>
    where
        T: DeserializeOwned + Send,
    {
        let got = self.get_bytes(now, key).await;

        match got {
            Cached::Err(e) => Cached::Err(e),
            Cached::Missing => Cached::Missing,
            Cached::Stale(bytes) => {
                let parsed = serde_json::from_slice(&bytes).map_err(|e| Error::new(e.to_string()));
                match parsed {
                    Ok(value) => Cached::Stale(value),
                    Err(e) => Cached::Err(e),
                }
            }
            Cached::Fresh(bytes) => {
                let parsed = serde_json::from_slice(&bytes).map_err(|e| Error::new(e.to_string()));
                match parsed {
                    Ok(value) => Cached::Fresh(value),
                    Err(e) => Cached::Err(e),
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
impl<T: Cache + ?Sized> CacheDbExt for T {}

pub type CacheDbDyn = Arc<dyn Cache>;
