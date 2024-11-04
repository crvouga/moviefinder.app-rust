use super::{
    impl_hash_map::ImplHashMap, impl_postgres::ImplPostgres, impl_with_cache, interface::KeyValueDb,
};
use crate::core::{db_conn_sql::interface::DbConnSql, logger::interface::Logger};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImplCachedPostgres<T: DbConnSql + 'static> {
    impl_with_cache: Box<dyn KeyValueDb>,
    impl_postgres: Arc<ImplPostgres<T>>,
    impl_hash_map: Arc<ImplHashMap>,
}

impl<T: DbConnSql + 'static> ImplCachedPostgres<T> {
    pub fn new(logger: Arc<dyn Logger>, db_conn_sql: Arc<T>) -> Self {
        let impl_postgres = Arc::new(ImplPostgres::new(logger.clone(), db_conn_sql.clone()));
        let impl_hash_map = Arc::new(ImplHashMap::new());
        let impl_with_cache = Box::new(impl_with_cache::ImplWithCache::new(
            impl_postgres.clone(),
            impl_hash_map.clone(),
        ));
        Self {
            impl_with_cache,
            impl_postgres: impl_postgres.clone(),
            impl_hash_map: impl_hash_map.clone(),
        }
    }
}

#[async_trait]
impl<T: DbConnSql> KeyValueDb for ImplCachedPostgres<T> {
    async fn get(&self, key: &str) -> Result<Option<String>, String> {
        self.impl_with_cache.get(key).await
    }

    async fn put(&self, key: &str, value: String) -> Result<(), String> {
        self.impl_with_cache.put(key, value).await
    }

    async fn zap(&self, key: &str) -> Result<(), String> {
        self.impl_with_cache.zap(key).await
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(Self {
            impl_with_cache: self.impl_with_cache.child(namespace.clone()),
            impl_hash_map: self.impl_hash_map.clone(),
            impl_postgres: self.impl_postgres.clone(),
        })
    }
}
