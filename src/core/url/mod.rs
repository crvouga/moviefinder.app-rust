use super::params::Params;

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
}
