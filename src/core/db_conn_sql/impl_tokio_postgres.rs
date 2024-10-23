use async_trait::async_trait;
use serde_json;
use tokio_postgres;

use super::interface::DbConnSql;

pub struct TokioPostgres {
    client: tokio_postgres::Client,
}

#[async_trait]
impl DbConnSql for TokioPostgres {
    async fn query<T, F>(&self, parse_row_json: F, query: &str) -> Result<Vec<T>, String>
    where
        F: Fn(String) -> Result<T, String> + Send + Sync,
    {
        let rows = self
            .client
            .query(query, &[])
            .await
            .map_err(|err| err.to_string())?;

        let mut results = vec![];

        for row in rows {
            let json = row_to_json(row)?;
            let parsed = parse_row_json(json)?;
            results.push(parsed);
        }

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
