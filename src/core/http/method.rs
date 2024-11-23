#[derive(Debug)]
pub enum HttpMethod {
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

impl HttpMethod {
    pub fn from_string(method: &str) -> Self {
        let cleaned = method.trim().to_uppercase();
        if cleaned.contains("GET") {
            HttpMethod::Get
        } else if cleaned.contains("POST") {
            HttpMethod::Post
        } else if cleaned.contains("PUT") {
            HttpMethod::Put
        } else if cleaned.contains("DELETE") {
            HttpMethod::Delete
        } else if cleaned.contains("PATCH") {
            HttpMethod::Patch
        } else if cleaned.contains("OPTIONS") {
            HttpMethod::Options
        } else if cleaned.contains("HEAD") {
            HttpMethod::Head
        } else if cleaned.contains("CONNECT") {
            HttpMethod::Connect
        } else if cleaned.contains("TRACE") {
            HttpMethod::Trace
        } else {
            HttpMethod::Get
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string(),
            HttpMethod::Patch => "PATCH".to_string(),
            HttpMethod::Options => "OPTIONS".to_string(),
            HttpMethod::Head => "HEAD".to_string(),
            HttpMethod::Connect => "CONNECT".to_string(),
            HttpMethod::Trace => "TRACE".to_string(),
        }
    }
}
