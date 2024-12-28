use super::interface::{CacheDbDyn, CacheDbExt, CacheResult};
use crate::core::{error::Error, unit_of_work::UnitOfWork};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, time::Duration};
use tokio::task;
///
///
///
///
///
///
pub async fn get_stale_while_revalidate<T, F, Fut>(
    uow: UnitOfWork,
    cache_db: CacheDbDyn,
    source: F,
    key: &str,
    ttl: Duration,
) -> Result<T, Error>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<T, Error>> + Send + 'static,
{
    let cached = cache_db.get_now::<T>(key).await;

    match cached {
        CacheResult::Fresh(value) => Ok(value),
        CacheResult::Stale(value) => {
            let cache_db_clone = cache_db.clone();
            let key = key.to_string();
            let ttl = ttl.clone();

            task::spawn(async move {
                match source().await {
                    Ok(new_value) => {
                        if let Err(e) = cache_db_clone.put_now(uow, ttl, &key, new_value).await {
                            eprintln!("Failed to update cache for key {}: {:?}", key, e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch fresh data for key {}: {:?}", key, e);
                    }
                }
            });

            Ok(value)
        }
        CacheResult::Missing => {
            let value = source().await?;

            cache_db.put_now(uow, ttl, key, value.clone()).await?;

            Ok(value)
        }
        CacheResult::Err(e) => Err(e),
    }
}
