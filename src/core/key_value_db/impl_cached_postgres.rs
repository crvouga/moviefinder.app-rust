use super::{
    impl_hash_map::HashMap, impl_postgres::Postgres, impl_with_cache, interface::KeyValueDb,
};
use crate::core::{
    db_conn_sql::interface::DbConnSqlDyn, logger::interface::LoggerDyn, unit_of_work::UnitOfWork,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CachedPostgres {
    impl_with_cache: Box<dyn KeyValueDb>,
    impl_postgres: Arc<Postgres>,
    impl_hash_map: Arc<HashMap>,
}

impl CachedPostgres {
    pub fn new(logger: LoggerDyn, db_conn_sql: DbConnSqlDyn) -> Self {
        let impl_postgres = Arc::new(Postgres::new(logger.clone(), db_conn_sql.clone()));
        let impl_hash_map = Arc::new(HashMap::new());
        let impl_with_cache = Box::new(impl_with_cache::WithCache::new(
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
impl KeyValueDb for CachedPostgres {
    async fn get(&self, key: &str) -> Result<Option<String>, std::io::Error> {
        self.impl_with_cache.get(key).await
    }

    async fn put(&self, uow: UnitOfWork, key: &str, value: String) -> Result<(), std::io::Error> {
        self.impl_with_cache.put(uow, key, value).await
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), std::io::Error> {
        self.impl_with_cache.zap(uow, key).await
    }

    fn child(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(Self {
            impl_with_cache: self.impl_with_cache.child(namespace.clone()),
            impl_hash_map: self.impl_hash_map.clone(),
            impl_postgres: self.impl_postgres.clone(),
        })
    }
}
