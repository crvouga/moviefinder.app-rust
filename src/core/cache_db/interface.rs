use crate::core::error::Error;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum GetStrategy {
    /// Use cache if available; otherwise fetch from source.
    /// If stale is detected, optionally refresh in background.
    CacheFirst,
    /// Always re-fetch from the source, optionally updating the cache if successful.
    SourceFirst,
    /// Serve stale data if the cache is expired, while asynchronously revalidating.
    StaleWhileRevalidate,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum PutStrategy {
    /// Update both cache and underlying store immediately (write-through).
    WriteThrough,
    /// Update cache now, persist asynchronously to store (write-back).
    WriteBack,
    /// Bypass cache and only write to store.
    NoCache,
    // etc.
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum DeleteStrategy {
    /// Remove from cache only.
    EvictFromCache,
    /// Remove from store only, leaving stale data in cache.
    EvictFromStore,
    /// Remove from both.
    EvictAll,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ClearStrategy {
    /// Clear the cache only.
    ClearCache,
    /// Clear the underlying store only.
    ClearStore,
    /// Clear both cache and store.
    ClearAll,
}

#[allow(dead_code)]
pub trait CacheDb {
    fn get_with_strategy(&self, key: &str, strategy: GetStrategy)
        -> Result<Option<Vec<u8>>, Error>;

    fn put_with_strategy(
        &self,
        key: &str,
        value: &[u8],
        strategy: PutStrategy,
    ) -> Result<(), Error>;

    fn delete_with_strategy(&self, key: &str, strategy: DeleteStrategy) -> Result<(), Error>;

    fn clear_with_strategy(&self, strategy: ClearStrategy) -> Result<(), Error>;

    // Optionally, keep simplified wrappers to avoid specifying a strategy every time:
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        self.get_with_strategy(key, GetStrategy::CacheFirst)
    }

    fn put(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        self.put_with_strategy(key, value, PutStrategy::WriteThrough)
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        self.delete_with_strategy(key, DeleteStrategy::EvictAll)
    }

    fn clear(&self) -> Result<(), Error> {
        self.clear_with_strategy(ClearStrategy::ClearAll)
    }
}
