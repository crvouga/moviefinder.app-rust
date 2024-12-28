use serde::{de::DeserializeOwned, Serialize};

use super::interface::{CacheDbDyn, CacheDbExt, Cached};
use crate::core::{error::Error, unit_of_work::UnitOfWork};
use std::{future::Future, pin::Pin, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CachedQueryStrategy {
    #[allow(dead_code)]
    StaleWhileRevalidate,
    StrictlyFresh,
}

impl Default for CachedQueryStrategy {
    fn default() -> Self {
        CachedQueryStrategy::StrictlyFresh
    }
}

pub struct CachedQuery<T>
where
    T: Serialize + DeserializeOwned + Sync + Send + Clone + 'static, // Added 'static lifetime
{
    uow: Option<UnitOfWork>,
    cache_db: Option<CacheDbDyn>,
    // Updated to store a closure returning a boxed Future
    query: Option<
        Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<T, Error>> + Send>> + Send + Sync>,
    >,
    key: String,
    ttl: Duration,
    strategy: CachedQueryStrategy, // Fixed typo from 'stragegy' to 'strategy'
}

impl<T> CachedQuery<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Clone + 'static, // Ensure 'static lifetime for futures
{
    pub fn new() -> Self {
        Self {
            uow: None,
            cache_db: None,
            query: None,
            key: String::new(),
            ttl: Duration::from_secs(60),
            strategy: CachedQueryStrategy::default(),
        }
    }

    pub fn uow(mut self, uow: UnitOfWork) -> Self {
        self.uow = Some(uow);
        self
    }

    pub fn cache_db(mut self, cache_db: CacheDbDyn) -> Self {
        self.cache_db = Some(cache_db);
        self
    }

    /// Accepts an asynchronous closure that returns a `Future`.
    pub fn query<F, Fut>(mut self, query: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, Error>> + Send + 'static,
    {
        self.query = Some(Box::new(move || Box::pin(query())));
        self
    }

    /// Optional helper method for clarity.
    pub fn query_async<F, Fut>(mut self, query: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, Error>> + Send + 'static,
    {
        self.query = Some(Box::new(move || Box::pin(query())));
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn strategy(mut self, strategy: CachedQueryStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn strategy_strictly_fresh(mut self) -> Self {
        self.strategy = CachedQueryStrategy::StrictlyFresh;
        self
    }

    /// Executes the cached query.
    pub async fn execute(self) -> Result<T, Error> {
        let cache_db = self
            .cache_db
            .ok_or_else(|| Error::new("Cache DB is not set"))?;
        let uow = self
            .uow
            .ok_or_else(|| Error::new("Unit of Work is not set"))?;
        let query = self.query.ok_or_else(|| Error::new("Query is not set"))?;

        match self.strategy {
            CachedQueryStrategy::StaleWhileRevalidate => {
                // Implement this strategy as needed
                unimplemented!()
            }

            CachedQueryStrategy::StrictlyFresh => {
                let cached = cache_db.get_now(&self.key).await;

                match cached {
                    Cached::Fresh(value) => Ok(value),
                    Cached::Err(e) => Err(e),
                    Cached::Stale(_) | Cached::Missing => {
                        let fresh_value = query().await?;
                        cache_db
                            .put_now(uow, self.ttl, &self.key, fresh_value.clone())
                            .await?;
                        Ok(fresh_value)
                    }
                }
            }
        }
    }
}
