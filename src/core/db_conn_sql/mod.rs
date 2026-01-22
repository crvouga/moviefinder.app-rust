use crate::core::sql::Sql;
use interface::DbConnSqlDyn;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use super::unit_of_work::UnitOfWork;

pub mod impl_noop;
pub mod impl_postgres;
pub mod interface;

pub async fn query<T>(
    db_conn: DbConnSqlDyn,
    query: &Sql,
) -> Result<Vec<T>, crate::core::error::CoreError>
where
    T: Debug + Send + Sync + DeserializeOwned, // DeserializeOwned allows deserialization directly
{
    let raw_rows = db_conn.query(query).await?;
    raw_rows
        .into_iter()
        .map(|value| {
            serde_json::from_value(value)
                .map_err(|e| crate::core::error::CoreError::new(e.to_string()))
        })
        .collect()
}

pub async fn execute(
    db_conn: DbConnSqlDyn,
    _uow: UnitOfWork,
    query: &Sql,
) -> Result<(), crate::core::error::CoreError> {
    db_conn.query(query).await.map(|_| ())
}
