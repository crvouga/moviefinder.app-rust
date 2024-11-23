use flate2::{write::GzEncoder, Compression};
use std::collections::HashMap;
use std::io::Write;

use super::content_encoding::ContentEncoding;
use super::request::HttpRequest;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn new(status_code: u16) -> HttpResponse {
        HttpResponse {
            status_code,
            body: Vec::new(),
            headers: HashMap::new(),
        }
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn to_body_string(self) -> String {
        String::from_utf8(self.body).unwrap_or_default()
    }

    pub fn to_http_string(&self) -> String {
        let headers_string = self
            .headers
            .iter()
            .fold(String::new(), |acc, (key, value)| {
                format!("{}{}: {}\r\n", acc, key, value)
            });

        let mut response = format!(
            "HTTP/1.1 {} {}\r\n{}",
            self.status_code,
            self.status_text(),
            headers_string
        );

        response.push_str("\r\n");
        response.push_str(&String::from_utf8_lossy(&self.body));

        response
    }

    pub fn to_http_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();

        let headers_string = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text());
        response.extend(headers_string.as_bytes());

        for (key, value) in &self.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            response.extend(header_line.as_bytes());
        }

        response.extend(b"\r\n");

        response.extend(&self.body);

        response
    }
    pub fn from_http_string(response: &str) -> Self {
        let mut lines = response.lines();
        let status_line = lines.next().unwrap_or("");
        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .parse::<u16>()
            .unwrap_or(500);

        let mut headers = HashMap::new();
        let mut body = Vec::new();
        let mut in_headers = true;

        for line in lines {
            if line.is_empty() {
                in_headers = false;
            } else if in_headers {
                if let Some((key, value)) = line.split_once(": ") {
                    headers.insert(key.to_string().to_ascii_lowercase(), value.to_string());
                }
            } else {
                body.extend_from_slice(line.as_bytes());
            }
        }

        HttpResponse {
            status_code,
            body,
            headers,
        }
    }

    fn status_text(&self) -> &'static str {
        match self.status_code {
            200 => "OK",
            204 => "No Content",
            302 => "Found",
            304 => "Not Modified",
            _ => "Unknown Status",
        }
    }

    pub fn compress(&mut self, request: &HttpRequest) {
        for encoding in request.to_accept_encoding() {
            match encoding {
                ContentEncoding::Gzip => {
                    self.compress_gzip();
                    break;
                }
                _ => {}
            }
        }
    }

    pub fn compress_gzip(&mut self) {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        encoder
            .write_all(&self.body)
            .expect("Failed to write to Gzip encoder");

        let compressed_body = encoder.finish().expect("Failed to finish Gzip encoding");

        self.body = compressed_body;

        self.headers
            .insert("Content-Encoding".to_string(), "gzip".to_string());

        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
    }
}
