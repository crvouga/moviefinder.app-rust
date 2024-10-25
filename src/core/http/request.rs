use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub form_data: HashMap<String, String>,
    #[allow(dead_code)]
    pub body: String,
}

impl HttpRequest {
    pub fn to_http_string(&self) -> String {
        let mut headers_string = String::new();
        for (key, value) in &self.headers {
            headers_string.push_str(&format!("{}: {}\r\n", key, value));
        }

        if !self
            .headers
            .iter()
            .any(|(key, _)| key.to_lowercase() == "host")
        {
            headers_string.push_str(&format!("Host: {}\r\n", self.host));
        }

        format!(
            "{} {} HTTP/1.1\r\n{}Connection: close\r\n\r\n",
            self.method, self.path, headers_string
        )
    }
}
