use serde::{de::DeserializeOwned, Serialize};

use super::interface::{CacheDbDyn, CacheDbExt, Cached};
use crate::core::{error::Error, unit_of_work::UnitOfWork};
use std::{fmt::Debug, future::Future, pin::Pin, time::Duration};

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
    T: Serialize + DeserializeOwned + Sync + Send + Clone + 'static,
{
    uow: Option<UnitOfWork>,
    cache_db: Option<CacheDbDyn>,
    query: Option<
        Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<T, Error>> + Send>> + Send + Sync>,
    >,
    key: String,
    max_age: Duration,
    strategy: CachedQueryStrategy,
}

impl<T> CachedQuery<T>
where
    T: Serialize + DeserializeOwned + Debug + Send + Sync + Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            uow: None,
            cache_db: None,
            query: None,
            key: String::new(),
            max_age: Duration::from_secs(60),
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

    pub fn query<F, Fut>(mut self, query: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, Error>> + Send + 'static,
    {
        self.query = Some(Box::new(move || Box::pin(query())));
        self
    }

    #[allow(dead_code)]
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

    pub fn max_age(mut self, max_age: Duration) -> Self {
        self.max_age = max_age;
        self
    }

    #[allow(dead_code)]
    pub fn strategy(mut self, strategy: CachedQueryStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn strategy_strictly_fresh(mut self) -> Self {
        self.strategy = CachedQueryStrategy::StrictlyFresh;
        self
    }

    pub async fn execute(self) -> Cached<T> {
        let cache_db = match self.cache_db {
            Some(cache_db) => cache_db,
            None => return Cached::Err(Error::new("CacheDb is not set")),
        };

        let uow = match self.uow {
            Some(uow) => uow,
            None => return Cached::Err(Error::new("UnitOfWork is not set")),
        };

        let query = match self.query {
            Some(query) => query,
            None => return Cached::Err(Error::new("Query is not set")),
        };

        match self.strategy {
            CachedQueryStrategy::StaleWhileRevalidate => {
                unimplemented!()
            }

            CachedQueryStrategy::StrictlyFresh => {
                let cached = cache_db.get_now(&self.key).await;

                match cached {
                    Cached::Err(e) => Cached::Err(e),
                    Cached::Fresh(value) => Cached::Fresh(value),
                    Cached::Stale(_value) => {
                        let queried = query().await;

                        let fresh_value = match queried.clone() {
                            Ok(value) => value,
                            Err(e) => return Cached::Err(e),
                        };

                        let put = cache_db
                            .put_now(uow, self.max_age, &self.key, fresh_value.clone())
                            .await;

                        match put {
                            Err(e) => Cached::Err(e),
                            Ok(_) => Cached::Fresh(fresh_value),
                        }
                    }
                    Cached::Missing => {
                        let queried = query().await;

                        let fresh_value = match queried.clone() {
                            Ok(value) => value,
                            Err(e) => return Cached::Err(e),
                        };

                        let put = cache_db
                            .put_now(uow, self.max_age, &self.key, fresh_value.clone())
                            .await;

                        match put {
                            Err(e) => Cached::Err(e),
                            Ok(_) => Cached::Fresh(fresh_value),
                        }
                    }
                }
            }
        }
    }
}
