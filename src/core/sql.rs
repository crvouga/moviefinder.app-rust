use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SqlVarType {
    Primitive(SqlPrimitive),
    Json(String),
    Commas(Vec<SqlPrimitive>),
}

#[derive(Debug, Clone)]
pub enum SqlPrimitive {
    Number(f64),
    Text(String),
    Boolean(bool),
    Null,
}

pub fn json<T: serde::Serialize>(value: T) -> SqlVarType {
    let json_str = serde_json::to_string(&value).expect("Failed to serialize to JSON");
    SqlVarType::Json(json_str)
}

pub fn commas(values: Vec<SqlPrimitive>) -> SqlVarType {
    SqlVarType::Commas(values)
}

pub type Vars = HashMap<String, SqlVarType>;

pub struct PaginationResult {
    pub query: String,
    pub vars: Vars,
}

pub fn pagination(limit: Option<u32>, offset: Option<u32>) -> PaginationResult {
    let mut query = String::new();
    let mut vars = Vars::new();

    if let Some(l) = limit {
        query.push_str(&format!("LIMIT {}\n", l));
        vars.insert(
            "limit".to_string(),
            SqlVarType::Primitive(SqlPrimitive::Number(l as f64)),
        );
    }

    if let Some(o) = offset {
        query.push_str(&format!("OFFSET {}\n", o));
        vars.insert(
            "offset".to_string(),
            SqlVarType::Primitive(SqlPrimitive::Number(o as f64)),
        );
    }

    PaginationResult { query, vars }
}

#[derive(Debug, Clone)]
pub struct Sql {
    query: String,
    vars: Vars,
}

impl Sql {
    pub fn new(query: &str) -> Self {
        Sql {
            query: query.to_string(),
            vars: HashMap::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut replaced = self.query.to_string();
        let mut keys: Vec<&String> = self.vars.keys().collect();

        keys.sort_by(|a, b| b.len().cmp(&a.len()));

        for key in keys {
            if let Some(value) = self.vars.get(key) {
                replaced = replace_param(&replaced, key, value);
            }
        }

        replaced
    }

    pub fn set(&mut self, key: &str, value: SqlVarType) {
        self.vars.insert(key.to_string(), value);
    }
}

fn replace_param(query: &str, variable: &str, value: &SqlVarType) -> String {
    let mut replaced = query.to_string();
    let placeholder = format!(":{}", variable);

    replaced = match value {
        SqlVarType::Primitive(SqlPrimitive::Number(n)) => {
            replaced.replace(&placeholder, &n.to_string())
        }
        SqlVarType::Primitive(SqlPrimitive::Text(s)) => {
            replaced.replace(&placeholder, &format!("'{}'", escape_quotes(s)))
        }
        SqlVarType::Primitive(SqlPrimitive::Boolean(b)) => {
            replaced.replace(&placeholder, &b.to_string())
        }
        SqlVarType::Primitive(SqlPrimitive::Null) => replaced.replace(&placeholder, "NULL"),
        SqlVarType::Json(json_str) => {
            replaced.replace(&placeholder, &format!("'{}'", escape_quotes(json_str)))
        }
        SqlVarType::Commas(values) => {
            let comma_str = values
                .iter()
                .map(|v| match v {
                    SqlPrimitive::Text(s) => format!("'{}'", escape_quotes(s)),
                    SqlPrimitive::Number(n) => n.to_string(),
                    SqlPrimitive::Boolean(b) => b.to_string(),
                    SqlPrimitive::Null => "NULL".to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ");
            replaced.replace(&placeholder, &comma_str)
        }
    };

    replaced
}

fn escape_quotes(input: &str) -> String {
    input.replace("'", "''")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_number() {
        let mut query = Sql::new("SELECT * FROM users WHERE id = :id");
        query.set("id", SqlVarType::Primitive(SqlPrimitive::Number(1.0)));

        let expected = "SELECT * FROM users WHERE id = 1";
        assert_eq!(query.to_string(), expected);
    }

    #[test]
    fn test_replace_text() {
        let mut query = Sql::new("SELECT * FROM users WHERE name = :name");
        query.set(
            "name",
            SqlVarType::Primitive(SqlPrimitive::Text("John".to_string())),
        );

        let expected = "SELECT * FROM users WHERE name = 'John'";
        assert_eq!(query.to_string(), expected);
    }

    #[test]
    fn test_replace_commas() {
        let mut query = Sql::new("SELECT * FROM users WHERE id IN (:ids)");
        query.set(
            "ids",
            SqlVarType::Commas(vec![
                SqlPrimitive::Number(1.0),
                SqlPrimitive::Number(2.0),
                SqlPrimitive::Number(3.0),
            ]),
        );

        let expected = "SELECT * FROM users WHERE id IN (1, 2, 3)";
        assert_eq!(query.to_string(), expected);
    }

    #[test]
    fn test_replace_json() {
        let mut query = Sql::new("INSERT INTO users_data (data) VALUES (:data)");
        query.set("data", json(&serde_json::json!({"key": "value"})));

        let expected = "INSERT INTO users_data (data) VALUES ('{\"key\":\"value\"}')";
        assert_eq!(query.to_string(), expected);
    }

    #[test]
    fn test_prevent_sql_injection() {
        let mut query = Sql::new("SELECT * FROM users WHERE name = :name");
        query.set(
            "name",
            SqlVarType::Primitive(SqlPrimitive::Text("John' OR 1=1 --".to_string())),
        );

        let expected = "SELECT * FROM users WHERE name = 'John'' OR 1=1 --'";
        assert_eq!(query.to_string(), expected);
    }
}
