use super::{response_writer::ResponseWriter, set_header::SetHeader};

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
