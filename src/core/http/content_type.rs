use super::request::Request;

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    ApplicationJson,
    ApplicationXWwwFormUrlencoded,
    MultipartFormData,
    Unknown,
}

impl ContentType {
    pub fn as_str(&self) -> &str {
        match self {
            ContentType::ApplicationJson => "application/json",
            ContentType::ApplicationXWwwFormUrlencoded => "application/x-www-form-urlencoded",
            ContentType::MultipartFormData => "multipart/form-data",
            ContentType::Unknown => "unknown",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "application/json" => Some(ContentType::ApplicationJson),
            "application/x-www-form-urlencoded" => Some(ContentType::ApplicationXWwwFormUrlencoded),
            "multipart/form-data" => Some(ContentType::MultipartFormData),
            _ => None,
        }
    }
}

impl Request {
    pub fn content_type(&self) -> ContentType {
        self.headers
            .get("content-type")
            .and_then(|value| ContentType::from_str(value))
            .unwrap_or(ContentType::Unknown)
    }
}
