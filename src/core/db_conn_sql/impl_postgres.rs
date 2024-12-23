use super::interface::DbConnSql;
use crate::{
    core::{logger::interface::LoggerDyn, sql::Sql},
    debug, error,
};
use async_trait::async_trait;
use serde_json::{self, Value};
use std::time::Duration;
use tokio_postgres::{self, Client, Error, NoTls};

pub struct Postgres {
    client: tokio_postgres::Client,
    simulate_latency: Option<Duration>,
    logger: LoggerDyn,
}

impl Postgres {
    pub async fn new(logger_parent: LoggerDyn, database_url: &str) -> Result<Self, String> {
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
    async fn query(&self, sql: &Sql) -> Result<Vec<Value>, crate::core::error::Error> {
        let start = std::time::Instant::now();

        if let Some(dur) = self.simulate_latency {
            tokio::time::sleep(dur).await;
        }

        let sql_str = sql.to_string();

        let rows = self
            .client
            .query(&sql_str, &[])
            .await
            .map_err(|err| crate::core::error::Error::new(err.to_string()))?;

        let mut results = vec![];

        let enum_types = fetch_enum_types(&self.client)
            .await
            .map_err(|err| crate::core::error::Error::new(err.to_string()))?;

        for row in rows {
            let json = row_to_json(row, &enum_types)?;
            results.push(json);
        }

        let dur = start.elapsed();

        debug!(self.logger, "\n\tsql={}\n\tduration={:?}", sql.query, dur);

        Ok(results)
    }
}

async fn fetch_enum_types(client: &Client) -> Result<std::collections::HashSet<String>, Error> {
    let rows = client
        .query(
            "SELECT t.typname AS type_name
             FROM pg_type t
             JOIN pg_enum e ON t.oid = e.enumtypid
             GROUP BY t.typname",
            &[],
        )
        .await?;

    let enum_types: std::collections::HashSet<String> = rows
        .into_iter()
        .map(|row| row.get::<_, String>("type_name"))
        .collect();

    Ok(enum_types)
}

fn row_to_json(
    row: tokio_postgres::Row,
    enum_types: &std::collections::HashSet<String>,
) -> Result<Value, crate::core::error::Error> {
    let mut json_obj = serde_json::Map::new();

    for (idx, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        let column_type = column.type_().name();

        let value: Value = if enum_types.contains(column_type) {
            // Handle enums as strings
            match row.try_get::<_, Option<String>>(idx) {
                Ok(Some(value)) => Value::String(value),
                Ok(None) => Value::Null,
                Err(err) => {
                    return Err(crate::core::error::Error::new(err.to_string()));
                }
            }
        } else {
            match column_type {
                // Handle strings
                "text" | "varchar" | "bpchar" => match row.try_get::<_, Option<String>>(idx) {
                    Ok(Some(value)) => Value::String(value),
                    Ok(None) => Value::Null,
                    Err(err) => {
                        return Err(crate::core::error::Error::new(err.to_string()));
                    }
                },
                // Handle integers
                "int2" | "int4" | "int8" => match row.try_get::<_, Option<i64>>(idx) {
                    Ok(Some(value)) => Value::Number(value.into()),
                    Ok(None) => Value::Null,
                    Err(err) => {
                        return Err(crate::core::error::Error::new(err.to_string()));
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
                        return Err(crate::core::error::Error::new(err.to_string()));
                    }
                },
                // Handle booleans
                "bool" => match row.try_get::<_, Option<bool>>(idx) {
                    Ok(Some(value)) => Value::Bool(value),
                    Ok(None) => Value::Null,
                    Err(err) => {
                        return Err(crate::core::error::Error::new(err.to_string()));
                    }
                },
                // Default case for unsupported types
                _ => Value::Null,
            }
        };

        json_obj.insert(column_name.to_string(), value);
    }

    Ok(Value::Object(json_obj))
}
