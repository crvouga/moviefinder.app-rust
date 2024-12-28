use super::{
    impl_hash_map::HashMap, impl_postgres::Postgres, impl_with_cache::WithCache,
    interface::KeyValueDb,
};
use crate::core::{
    db_conn_sql::interface::DbConnSqlDyn, error::Error, logger::interface::LoggerDyn,
    unit_of_work::UnitOfWork,
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
        let impl_postgres = Arc::new(Postgres::new(logger.clone(), db_conn_sql));
        let impl_hash_map = Arc::new(HashMap::new());
        let impl_with_cache =
            Box::new(WithCache::new(impl_postgres.clone(), impl_hash_map.clone()));
        Self {
            impl_with_cache,
            impl_postgres,
            impl_hash_map,
        }
    }
}

#[async_trait]
impl KeyValueDb for CachedPostgres {
    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        self.impl_with_cache.get_bytes(key).await
    }

    async fn put_bytes(&self, uow: UnitOfWork, key: &str, value: &[u8]) -> Result<(), Error> {
        self.impl_with_cache.put_bytes(uow, key, value).await
    }

    async fn zap(&self, uow: UnitOfWork, key: &str) -> Result<(), Error> {
        self.impl_with_cache.zap(uow, key).await
    }

    fn namespace(&self, namespace: Vec<String>) -> Box<dyn KeyValueDb> {
        Box::new(Self {
            impl_with_cache: self.impl_with_cache.namespace(namespace.clone()),
            impl_postgres: self.impl_postgres.clone(),
            impl_hash_map: self.impl_hash_map.clone(),
        })
    }
}
