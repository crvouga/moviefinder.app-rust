use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

use super::content_encoding::ContentEncoding;
use super::response_writer::ResponseWriter;
use super::set_header::SetHeader;

impl ResponseWriter {
    pub async fn write_body_with_compression(&mut self, body: &[u8]) -> Result<(), std::io::Error> {
        for encoding in &self.content_encodings {
            match encoding {
                ContentEncoding::Gzip => {
                    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                    encoder
                        .write_all(body)
                        .expect("Failed to write to Gzip encoder");
                    let body_new = encoder.finish().expect("Failed to finish Gzip encoding");
                    self.set_header("Content-Encoding", "gzip");
                    return self.write_body(body_new.as_slice()).await;
                }
                _ => {
                    continue;
                }
            }
        }

        self.write_body(body).await
    }
}
