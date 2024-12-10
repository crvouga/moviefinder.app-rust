use super::unstructured_data::UnstructuredData;

pub mod query_params;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Url {
    pub host: String,
    pub path: String,
    pub query_params: query_params::QueryParams,
}

impl Url {
    #[allow(dead_code)]
    pub fn from_str(url: &str) -> Result<Url, String> {
        let url_parts: Vec<&str> = url.split('?').collect();

        if url_parts.is_empty() {
            return Err("Invalid URL: Missing host and path.".to_string());
        }

        let host_and_path: Vec<&str> = url_parts[0].split('/').collect();
        if host_and_path.len() < 3 {
            return Err("Invalid URL: Incomplete host or path.".to_string());
        }

        let host = host_and_path
            .get(2)
            .map(|s| s.to_string())
            .ok_or_else(|| "Invalid URL: Missing host.".to_string())?;

        let path = host_and_path.get(3..).unwrap_or(&[]).join("/");

        let query_params = match url_parts.get(1) {
            Some(query_params_str) => query_params::QueryParams::from_string(query_params_str),
            None => query_params::QueryParams::default(),
        };

        Ok(Url {
            host,
            path,
            query_params,
        })
    }

    pub fn to_string(&self) -> String {
        if self.query_params.is_empty() {
            format!("https://{}/{}", self.host, self.path)
        } else {
            format!(
                "https://{}/{}?{}",
                self.host,
                self.path,
                self.query_params.to_string()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::query_params::QueryParams;
    use super::*;

    #[test]
    fn test_to_string_without_query_params() {
        let url = Url {
            host: "example.com".to_string(),
            path: "path/to/resource".to_string(),
            query_params: QueryParams::default(),
        };

        assert_eq!(url.to_string(), "https://example.com/path/to/resource");
    }

    #[test]
    fn test_to_string_with_query_params() {
        let mut query_params = QueryParams::default();
        query_params.insert(&"key1".to_string(), "value1".to_string());
        query_params.insert(&"key2".to_string(), "value2".to_string());

        let url = Url {
            host: "example.com".to_string(),
            path: "path/to/resource".to_string(),
            query_params,
        };

        let result = url.to_string();
        let expected_1 = "https://example.com/path/to/resource?key1=value1&key2=value2";
        let expected_2 = "https://example.com/path/to/resource?key2=value2&key1=value1";

        assert!(result == expected_1 || result == expected_2);
    }
}
