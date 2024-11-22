use std::collections::HashMap;

use super::{content_encoding::ContentEncoding, form_data::FormData, query_params::QueryParams};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub form_data: FormData,
    pub query_params: QueryParams,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn to_http_string(&self) -> String {
        let mut headers_string = String::new();
        for (key, value) in &self.headers {
            headers_string.push_str(&format!("{}: {}\r\n", key, value));
        }

        if !self
            .headers
            .iter()
            .any(|(key, _)| key.to_lowercase() == "host")
        {
            headers_string.push_str(&format!("Host: {}\r\n", self.host));
        }

        let path_with_query_params = if self.query_params.is_empty() {
            self.path.clone()
        } else {
            format!("{}?{}", self.path, self.query_params.to_string())
        };

        format!(
            "{} {} HTTP/1.1\r\n{}Connection: close\r\n\r\n",
            self.method, path_with_query_params, headers_string
        )
    }

    pub fn to_accept_encoding(&self) -> Vec<ContentEncoding> {
        self.headers
            .get("Accept-Encoding")
            .unwrap_or(&"".to_string())
            .split(",")
            .map(ContentEncoding::from_str)
            .collect::<Vec<ContentEncoding>>()
    }
}
