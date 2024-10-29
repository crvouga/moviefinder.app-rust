use std::fmt::Debug;

use async_trait::async_trait;
use serde_json;
use tokio_postgres::{self, NoTls};

use crate::core::sql::Sql;

use super::interface::DbConnSql;

pub struct ImplPostgres {
    client: tokio_postgres::Client,
    simulate_latency: bool,
}

impl ImplPostgres {
    pub async fn new(database_url: &str) -> Result<Self, String> {
        let (client, connection) = tokio_postgres::connect(database_url, NoTls)
            .await
            .map_err(|err| err.to_string())?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("Database connection error: {}", err);
            }
        });

        Ok(Self {
            client,
            simulate_latency: false,
        })
    }

    pub fn simulate_latency(mut self, simulate_latency: bool) -> Self {
        self.simulate_latency = simulate_latency;
        self
    }
}

#[async_trait]
impl DbConnSql for ImplPostgres {
    async fn query<T, F>(&self, parse_row_json: Box<F>, sql: &Sql) -> Result<Vec<T>, String>
    where
        F: Fn(String) -> Result<T, String> + Send + Sync,
        T: Debug,
    {
        let sql_str = sql.to_string();

        let rows = self
            .client
            .query(&sql_str, &[])
            .await
            .map_err(|err| err.to_string())?;

        let mut results = vec![];

        for row in rows {
            let json = row_to_json(row)?;
            let parsed = parse_row_json(json)?;
            results.push(parsed);
        }

        if false {
            println!("LOG\n\t{:?}\n\t{:?}", sql, results);
        }

        // if self.simulate_latency {
        //     tokio::time::sleep(std::time::Duration::from_secs(100)).await;
        // }

        Ok(results)
    }
}

fn row_to_json(row: tokio_postgres::Row) -> Result<String, String> {
    let mut json_obj = serde_json::Map::new();

    for (idx, column) in row.columns().iter().enumerate() {
        let column_name = column.name();
        let value: serde_json::Value = match row.get(idx) {
            Some(value) => serde_json::Value::String(value),
            None => serde_json::Value::Null,
        };
        json_obj.insert(column_name.to_string(), value);
    }

    serde_json::to_string(&json_obj).map_err(|e| e.to_string())
}
