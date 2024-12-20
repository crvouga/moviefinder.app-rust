use crate::core::sql::Sql;
use interface::DbConnSqlDyn;
use serde_json::Value;
use std::{fmt::Debug, io::Error};

use super::unit_of_work::UnitOfWork;

pub mod impl_noop;
pub mod impl_postgres;
pub mod interface;

pub async fn query<T, F>(db_conn: DbConnSqlDyn, query: &Sql, parse_row: F) -> Result<Vec<T>, Error>
where
    F: Fn(Value) -> Result<T, Error>,
    T: Debug + Send + Sync,
{
    let raw_rows = db_conn.query(query).await?;
    raw_rows
        .into_iter()
        .map(parse_row)
        .collect::<Result<Vec<T>, Error>>()
}

pub async fn execute(db_conn: DbConnSqlDyn, _uow: UnitOfWork, query: &Sql) -> Result<(), Error> {
    db_conn.query(query).await.map(|_| ())
}
