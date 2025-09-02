use super::interface::DbConnSql;
use crate::{
    core::{error::CoreError, logger::interface::LoggerDyn, sql::Sql},
    debug, error, info,
};
use async_trait::async_trait;
use serde_json::{self, Value};
use std::time::Duration;
use tokio_postgres::{self, Client, Error, NoTls};

pub struct Postgres {
    client: tokio_postgres::Client,
    simulate_latency: Option<Duration>,
    logger: LoggerDyn,
    timeout: Duration,
    max_retries: u32,
}

impl Postgres {
    pub async fn new(logger_parent: LoggerDyn, database_url: &str) -> Result<Self, String> {
        if database_url.trim().is_empty() {
            return Err("Database URL is empty".to_string());
        }

        let timeout = Duration::from_secs(30);
        let max_retries = 10;
        let mut last_error = None;
        let mut retry_count = 0;

        info!(logger_parent, "Connecting to postgres database...");

        while retry_count < max_retries {
            if retry_count > 0 {
                let backoff = Duration::from_millis(50 * (2_u64.pow(retry_count - 1)));
                info!(
                    logger_parent,
                    "Waiting {:?} before retry {}/{}",
                    backoff,
                    retry_count + 1,
                    max_retries
                );
                tokio::time::sleep(backoff).await;
            }

            info!(
                logger_parent,
                "Attempting database connection (attempt {}/{})",
                retry_count + 1,
                max_retries
            );

            match tokio::time::timeout(timeout, tokio_postgres::connect(database_url, NoTls)).await
            {
                Ok(connect_result) => match connect_result {
                    Ok((client, connection)) => {
                        let logger = logger_parent.child("postgres");

                        info!(logger, "Successfully connected to database");

                        tokio::spawn(async move {
                            if let Err(err) = connection.await {
                                error!(logger, "Database connection error: {}", err);
                            }
                        });

                        return Ok(Self {
                            client,
                            logger: logger_parent.child("postgres"),
                            simulate_latency: None,
                            timeout,
                            max_retries,
                        });
                    }
                    Err(e) => {
                        error!(
                            logger_parent,
                            "Failed to connect to database: {}",
                            e.to_string()
                        );
                        last_error = Some(e.to_string());
                        retry_count += 1;
                    }
                },
                Err(_) => {
                    error!(logger_parent, "Connection attempt timed out");
                    last_error = Some("Connection timeout".to_string());
                    retry_count += 1;
                }
            }
        }

        error!(
            logger_parent,
            "Failed to connect to database after {} attempts", max_retries
        );
        Err(last_error.unwrap_or_else(|| "Failed to connect to database".to_string()))
    }

    pub fn simulate_latency(mut self, simulate_latency: Option<Duration>) -> Self {
        if let Some(latency) = simulate_latency {
            info!(self.logger, "Simulating latency of {:?}", latency);
        }
        self.simulate_latency = simulate_latency;
        self
    }
}

#[async_trait]
impl DbConnSql for Postgres {
    async fn query(&self, sql: &Sql) -> Result<Vec<Value>, crate::core::error::CoreError> {
        let start = std::time::Instant::now();

        debug!(self.logger, "Executing SQL query: {}", sql.query);

        if let Some(dur) = self.simulate_latency {
            debug!(self.logger, "Simulating latency of {:?}", dur);
            tokio::time::sleep(dur).await;
        }

        let sql_str = sql.to_string();
        let mut last_error: Option<CoreError> = None;
        let mut retry_count = 0;

        while retry_count < self.max_retries {
            match tokio::time::timeout(self.timeout, self.client.query(&sql_str, &[])).await {
                Ok(query_result) => match query_result {
                    Ok(rows) => {
                        let mut results = vec![];

                        debug!(self.logger, "Fetching enum types...");
                        let enum_types = fetch_enum_types(&self.client)
                            .await
                            .map_err(|err| crate::core::error::CoreError::new(err.to_string()))?;

                        debug!(self.logger, "Converting {} rows to JSON", rows.len());
                        for row in rows {
                            let json = row_to_json(row, &enum_types)?;
                            results.push(json);
                        }

                        let dur = start.elapsed();
                        debug!(
                            self.logger,
                            "\n\tsql={}\n\trows={}\n\tduration={:?}",
                            sql.query,
                            results.len(),
                            dur
                        );

                        return Ok(results);
                    }
                    Err(err) => {
                        error!(self.logger, "Query failed: {}", err);
                        last_error = Some(CoreError::new(err.to_string()));
                        retry_count += 1;
                        if retry_count < self.max_retries {
                            debug!(
                                self.logger,
                                "Query failed, retrying ({}/{}): {}",
                                retry_count,
                                self.max_retries,
                                err
                            );
                            tokio::time::sleep(Duration::from_millis(
                                100 * (2_u64.pow(retry_count)),
                            ))
                            .await;
                        }
                    }
                },
                Err(_) => {
                    error!(self.logger, "Query timed out after {:?}", self.timeout);
                    last_error = Some(CoreError::new("Query timed out".to_string()));
                    retry_count += 1;
                    if retry_count < self.max_retries {
                        debug!(
                            self.logger,
                            "Query timed out, retrying ({}/{})", retry_count, self.max_retries
                        );
                        tokio::time::sleep(Duration::from_millis(100 * (2_u64.pow(retry_count))))
                            .await;
                    }
                }
            }
        }

        error!(
            self.logger,
            "Query failed after {} retries", self.max_retries
        );
        Err(crate::core::error::CoreError::new(
            last_error
                .map(|e| e.to_string())
                .unwrap_or_else(|| "Query failed after retries".to_string()),
        ))
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
) -> Result<Value, crate::core::error::CoreError> {
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
                    return Err(crate::core::error::CoreError::new(err.to_string()));
                }
            }
        } else {
            match column_type {
                // Handle strings
                "text" | "varchar" | "bpchar" => match row.try_get::<_, Option<String>>(idx) {
                    Ok(Some(value)) => Value::String(value),
                    Ok(None) => Value::Null,
                    Err(err) => {
                        return Err(crate::core::error::CoreError::new(err.to_string()));
                    }
                },
                // Handle integers
                "int2" | "int4" | "int8" => match row.try_get::<_, Option<i64>>(idx) {
                    Ok(Some(value)) => Value::Number(value.into()),
                    Ok(None) => Value::Null,
                    Err(err) => {
                        return Err(crate::core::error::CoreError::new(err.to_string()));
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
                        return Err(crate::core::error::CoreError::new(err.to_string()));
                    }
                },
                // Handle booleans
                "bool" => match row.try_get::<_, Option<bool>>(idx) {
                    Ok(Some(value)) => Value::Bool(value),
                    Ok(None) => Value::Null,
                    Err(err) => {
                        return Err(crate::core::error::CoreError::new(err.to_string()));
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
