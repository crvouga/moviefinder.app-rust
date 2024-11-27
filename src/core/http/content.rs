use super::{request::Request, response_writer::ResponseWriter, set_header::SetHeader};

impl Request {
    pub fn is_accepting(&self, content_type: &str) -> bool {
        self.headers
            .get("accept")
            .unwrap_or(&"".to_string())
            .to_lowercase()
            .contains(&content_type.to_lowercase())
    }

    pub fn is_accepting_html(&self) -> bool {
        self.is_accepting("html")
    }
}

impl ResponseWriter {
    pub async fn content(&mut self, content_type: &str, body: &[u8]) -> Result<(), std::io::Error> {
        self.set_header("Content-Type", content_type);
        self.write_body(body).await
    }

    pub async fn text(&mut self, body: &str) -> Result<(), std::io::Error> {
        self.content("text/plain", body.as_bytes()).await
    }

    pub async fn css(&mut self, body: &[u8]) -> Result<(), std::io::Error> {
        self.content("text/css", body).await
    }

    pub async fn html(&mut self, body: &str) -> Result<(), std::io::Error> {
        self.content("text/html", body.as_bytes()).await
    }
}
