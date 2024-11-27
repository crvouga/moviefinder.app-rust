use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::set_header::SetHeader;

pub struct ResponseWriter {
    pub stream: TcpStream,
    pub headers_sent: bool,
    pub initial_headers: HashMap<String, String>,
}

impl ResponseWriter {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            headers_sent: false,
            initial_headers: HashMap::new(),
        }
    }

    pub async fn write_headers(&mut self) -> Result<(), std::io::Error> {
        if self.headers_sent {
            return Ok(());
        }

        let headers = if self.initial_headers.is_empty() {
            let mut default_headers = HashMap::new();
            default_headers.insert("content-type".to_string(), "text/plain".to_string());
            default_headers
        } else {
            self.initial_headers.clone()
        };

        let mut header_string = "HTTP/1.1 200 OK\r\n".to_string();
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
