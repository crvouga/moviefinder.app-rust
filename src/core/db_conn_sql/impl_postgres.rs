use std::{fmt::Debug, sync::Arc, time::Duration};

use async_trait::async_trait;
use serde_json;
use tokio_postgres::{self, NoTls};

use crate::{
    core::{logger::interface::Logger, sql::Sql},
    debug, error,
};

use super::interface::DbConnSql;

pub struct Postgres {
    client: tokio_postgres::Client,
    simulate_latency: Option<Duration>,
    logger: Arc<dyn Logger>,
}

impl Postgres {
    pub async fn new(logger_parent: Arc<dyn Logger>, database_url: &str) -> Result<Self, String> {
        let (client, connection) = tokio_postgres::connect(database_url, NoTls)
            .await
            .map_err(|err| err.to_string())?;

        let logger = logger_parent.child("postgres");

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                error!(logger, "Database connection error: {}", err);
            }
        });

        Ok(Self {
            client,
            logger: logger_parent.child("postgres"),
            simulate_latency: None,
        })
    }

    pub fn simulate_latency(mut self, simulate_latency: Option<Duration>) -> Self {
        self.simulate_latency = simulate_latency;
        self
    }
}

#[async_trait]
impl DbConnSql for Postgres {
    async fn query<T, F>(&self, parse_row_json: Box<F>, sql: &Sql) -> Result<Vec<T>, std::io::Error>
    where
        F: Fn(String) -> Result<T, std::io::Error> + Send + Sync,
        T: Debug,
    {
        let start = std::time::Instant::now();

        if let Some(dur) = self.simulate_latency {
            tokio::time::sleep(dur).await;
        }

        let sql_str = sql.to_string();

        let rows = self
            .client
            .query(&sql_str, &[])
            .await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

        let mut results = vec![];

        for row in rows {
            let json = row_to_json(row)?;
            let parsed = parse_row_json(json)?;
            results.push(parsed);
        }

        let dur = start.elapsed();

        debug!(self.logger, "\n\tsql={}\n\tduration={:?}", sql.query, dur);

        Ok(results)
    }
}

fn row_to_json(row: tokio_postgres::Row) -> Result<String, std::io::Error> {
    let mut json_obj = serde_json::Map::new();

    for (idx, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        let value: serde_json::Value = match row.get(idx) {
            Some(value) => serde_json::Value::String(value),
            None => serde_json::Value::Null,
        };
        json_obj.insert(column_name.to_string(), value);
    }

    serde_json::to_string(&json_obj)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
