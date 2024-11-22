pub enum ContentEncoding {
    Identity,
    Gzip,
    Deflate,
}

impl ContentEncoding {
    pub fn from_str(value: &str) -> ContentEncoding {
        let cleaned_value = value.trim().to_ascii_lowercase();
        if cleaned_value.contains("gzip") {
            return ContentEncoding::Gzip;
        }
        if cleaned_value.contains("deflate") {
            return ContentEncoding::Deflate;
        }
        if cleaned_value.contains("identity") {
            return ContentEncoding::Identity;
        }
        ContentEncoding::Identity
    }
}
