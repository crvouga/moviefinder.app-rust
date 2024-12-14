use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use serde_json::{self, Value};
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
        if database_url.trim().is_empty() {
            return Err("Database URL is empty".to_string());
        }

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
    async fn query(&self, sql: &Sql) -> Result<Vec<Value>, std::io::Error> {
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
            results.push(json);
        }

        let dur = start.elapsed();

        debug!(self.logger, "\n\tsql={}\n\tduration={:?}", sql.query, dur);

        Ok(results)
    }
}

fn row_to_json(row: tokio_postgres::Row) -> Result<Value, std::io::Error> {
    let mut json_obj = serde_json::Map::new();

    for (idx, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        let value: Value = match column.type_().name() {
            // PostgreSQL enums are returned as strings, so we directly try to get them as strings
            "enum" | "text" | "varchar" | "bpchar" => match row.try_get::<_, Option<String>>(idx) {
                Ok(Some(value)) => Value::String(value),
                Ok(None) => Value::Null,
                Err(err) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    ));
                }
            },
            // Handle integers
            "int2" | "int4" | "int8" => match row.try_get::<_, Option<i64>>(idx) {
                Ok(Some(value)) => Value::Number(value.into()),
                Ok(None) => Value::Null,
                Err(err) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    ));
                }
            },
            // Handle floating-point numbers
            "float4" | "float8" => match row.try_get::<_, Option<f64>>(idx) {
                Ok(Some(value)) => {
                    if let Some(number) = serde_json::Number::from_f64(value) {
                        Value::Number(number)
                    } else {
                        Value::Null
                    }
                }
                Ok(None) => Value::Null,
                Err(err) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    ));
                }
            },
            // Handle booleans
            "bool" => match row.try_get::<_, Option<bool>>(idx) {
                Ok(Some(value)) => Value::Bool(value),
                Ok(None) => Value::Null,
                Err(err) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    ));
                }
            },
            // Default case for unsupported types
            _ => Value::Null,
        };
        json_obj.insert(column_name.to_string(), value);
    }

    Ok(Value::Object(json_obj))
}
