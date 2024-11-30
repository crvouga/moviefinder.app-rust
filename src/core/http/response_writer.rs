use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::set_header::SetHeader;

pub struct ResponseWriter {
    pub stream: TcpStream,
    pub headers_sent: bool,
    pub initial_headers: HashMap<String, String>,
    pub status_code: u16,
}

impl ResponseWriter {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            status_code: 200,
            stream,
            headers_sent: false,
            initial_headers: HashMap::new(),
        }
    }

    pub fn status_code(&mut self, status_code: u16) -> &Self {
        self.status_code = status_code;
        self
    }

    pub async fn write_headers(&mut self) -> Result<(), std::io::Error> {
        if self.headers_sent {
            return Ok(());
        }

        let reason_phrase = match self.status_code {
            200 => "OK",
            301 => "Moved Permanently",
            302 => "Found",
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        };

        let mut header_string = format!("HTTP/1.1 {} {}\r\n", self.status_code, reason_phrase);

        let headers = if self.initial_headers.is_empty() {
            let mut default_headers = HashMap::new();
            default_headers.insert("content-type".to_string(), "text/plain".to_string());
            default_headers
        } else {
            self.initial_headers.clone()
        };

        for (key, value) in headers {
            header_string.push_str(&format!("{}: {}\r\n", key, value));
        }
        header_string.push_str("\r\n");

        self.stream.write_all(header_string.as_bytes()).await?;
        self.headers_sent = true;
        Ok(())
    }

    pub async fn write_body(&mut self, body: &[u8]) -> Result<(), std::io::Error> {
        if !self.headers_sent {
            self.write_headers().await?;
        }

        self.stream.write_all(body).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn end(&mut self) -> Result<(), std::io::Error> {
        if !self.headers_sent {
            self.write_headers().await?;
        }
        self.stream.flush().await
    }
}

impl SetHeader for ResponseWriter {
    fn set_header(&mut self, key: &str, value: &str) -> &Self {
        self.initial_headers
            .insert(key.to_string(), value.to_string());
        self
    }
}
