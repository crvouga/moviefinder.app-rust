#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Connect,
    Trace,
}

impl Method {
    pub fn from_string(method: &str) -> Self {
        let cleaned = method.trim().to_uppercase();
        if cleaned.contains("GET") {
            Method::Get
        } else if cleaned.contains("POST") {
            Method::Post
        } else if cleaned.contains("PUT") {
            Method::Put
        } else if cleaned.contains("DELETE") {
            Method::Delete
        } else if cleaned.contains("PATCH") {
            Method::Patch
        } else if cleaned.contains("OPTIONS") {
            Method::Options
        } else if cleaned.contains("HEAD") {
            Method::Head
        } else if cleaned.contains("CONNECT") {
            Method::Connect
        } else if cleaned.contains("TRACE") {
            Method::Trace
        } else {
            Method::Get
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Method::Get => "GET".to_string(),
            Method::Post => "POST".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Delete => "DELETE".to_string(),
            Method::Patch => "PATCH".to_string(),
            Method::Options => "OPTIONS".to_string(),
            Method::Head => "HEAD".to_string(),
            Method::Connect => "CONNECT".to_string(),
            Method::Trace => "TRACE".to_string(),
        }
    }
}
