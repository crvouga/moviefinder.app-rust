use super::{
    content_encoding::ContentEncoding, request::Request, response_writer::ResponseWriter,
    set_header::SetHeader,
};

impl Request {}

impl ResponseWriter {
    pub async fn content(
        &mut self,
        content_type: &str,
        content_encodings: Vec<ContentEncoding>,
        body: &[u8],
    ) -> Result<(), std::io::Error> {
        self.set_header("Content-Type", content_type);
        self.write_body_with_compression(content_encodings, body)
            .await
    }
}
