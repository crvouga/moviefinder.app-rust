#[derive(Debug, PartialEq, Eq)]
pub enum ContentEncoding {
    Identity,
    Gzip,
    Deflate,
}

impl ContentEncoding {
    pub fn from_str(value: &str) -> ContentEncoding {
        let cleaned_value = value.trim().to_ascii_lowercase();

        let encoding: ContentEncoding = if cleaned_value.contains("gzip") {
            ContentEncoding::Gzip
        } else if cleaned_value.contains("deflate") {
            ContentEncoding::Deflate
        } else if cleaned_value.contains("identity") {
            ContentEncoding::Identity
        } else {
            ContentEncoding::Identity
        };

        encoding
    }
}
