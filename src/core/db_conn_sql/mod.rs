use crate::core::sql::Sql;
use interface::DbConnSqlDyn;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, io::Error};

use super::unit_of_work::UnitOfWork;

pub mod impl_noop;
pub mod impl_postgres;
pub mod interface;

pub async fn query<T>(db_conn: DbConnSqlDyn, query: &Sql) -> Result<Vec<T>, Error>
where
    T: Debug + Send + Sync + DeserializeOwned, // DeserializeOwned allows deserialization directly
{
    let raw_rows = db_conn.query(query).await?;
    raw_rows
        .into_iter()
        .map(|value| {
            serde_json::from_value(value)
                .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))
        })
        .collect()
}

pub async fn execute(db_conn: DbConnSqlDyn, _uow: UnitOfWork, query: &Sql) -> Result<(), Error> {
    db_conn.query(query).await.map(|_| ())
}
