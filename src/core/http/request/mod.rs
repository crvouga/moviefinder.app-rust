use super::{content_encoding::ContentEncoding, form_data::FormData, method::Method};
use crate::core::{unstructed_data::UnstructedData, url::Url};
use std::collections::HashMap;

mod request_test;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub form_data: FormData,
    pub body: Vec<u8>,
}

impl Request {
    #[allow(dead_code)]
    pub fn to_http_string(&self) -> String {
        let mut headers_vec: Vec<_> = self.headers.iter().collect();
        headers_vec.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        let mut headers_string = String::new();
        for (key, value) in headers_vec {
            headers_string.push_str(&format!("{}: {}\r\n", key, value));
        }

        if !self
            .headers
            .iter()
            .any(|(key, _)| key.to_lowercase() == "host")
        {
            headers_string.push_str(&format!("Host: {}\r\n", self.url.host));
        }

        let path_with_query_params = if self.url.query_params.is_empty() {
            self.url.path.clone()
        } else {
            format!("{}?{}", self.url.path, self.url.query_params.to_string())
        };

        let body_string = String::from_utf8_lossy(&self.body);

        format!(
            "{} {} HTTP/1.1\r\n{}Connection: close\r\n\r\n{}",
            self.method.to_string(),
            path_with_query_params,
            headers_string,
            body_string
        )
    }

    pub fn to_accept_encoding(&self) -> Vec<ContentEncoding> {
        self.headers
            .get("accept-encoding")
            .unwrap_or(&"".to_string())
            .split(",")
            .map(ContentEncoding::from_str)
            .collect::<Vec<ContentEncoding>>()
    }
}
