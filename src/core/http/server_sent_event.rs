use super::{header::SetHeader, response_writer::HttpResponseWriter};
use tokio::io::AsyncWriteExt;

impl HttpResponseWriter {
    pub async fn write_sse_event(
        &mut self,
        event: &str,
        data: Vec<&str>,
    ) -> Result<(), std::io::Error> {
        if !self.headers_sent {
            self.set_header("content-type", "text/event-stream");
            self.set_header("cache-control", "no-cache");
            self.set_header("connection", "keep-alive");
            self.write_headers().await?;
        }

        let mut sse_message = String::new();
        if !event.is_empty() {
            sse_message.push_str(&format!("event: {}\n", event));
        }
        for d in data {
            sse_message.push_str(&format!("data: {}\n", d));
        }

        println!("sse_message:\n{}", sse_message);

        self.stream.write_all(sse_message.as_bytes()).await?;
        self.stream.flush().await?;
        Ok(())
    }
}

pub struct ServerSentEvent {
    event: String,
    data: Vec<String>,
}

pub fn sse() -> ServerSentEvent {
    ServerSentEvent::new()
}

impl ServerSentEvent {
    pub fn new() -> ServerSentEvent {
        ServerSentEvent {
            event: String::new(),
            data: Vec::new(),
        }
    }
    pub fn event(&mut self, event: &str) -> &mut Self {
        self.event = event.to_string();
        self
    }

    pub fn data(&mut self, data: &str) -> &mut Self {
        self.data.push(data.to_string());
        self
    }

    pub async fn send(&mut self, response_writer: &mut HttpResponseWriter) {
        let _ = response_writer
            .write_sse_event(&self.event, self.data.iter().map(|s| s.as_str()).collect())
            .await;
    }
}