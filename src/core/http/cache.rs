use super::{response_writer::ResponseWriter, set_header::SetHeader};

impl ResponseWriter {
    pub fn set_long_term_cache(&mut self) {
        self.set_header("cache-control", "public, max-age=31536000, immutable");
    }
}
