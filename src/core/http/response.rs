use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn new(status_code: u16, body: String, headers: HashMap<String, String>) -> HttpResponse {
        HttpResponse {
            status_code,
            body,
            headers,
        }
    }

    pub fn to_http_string(&self) -> String {
        let headers_string = self
            .headers
            .iter()
            .fold(String::new(), |acc, (key, value)| {
                format!("{}{}: {}\r\n", acc, key, value)
            });

        let http_string = format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_text(),
            headers_string,
            self.body.len(),
            self.body
        );

        println!("{}", headers_string);

        http_string
    }

    pub fn from_http_string(response: &str) -> Self {
        let mut lines = response.lines();
        let status_line = lines.next().unwrap_or("");
        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .parse::<u16>()
            .unwrap_or(500);

        let mut headers = HashMap::new();
        let mut body = String::new();
        let mut in_headers = true;

        for line in lines {
            if line.is_empty() {
                in_headers = false;
            } else if in_headers {
                if let Some((key, value)) = line.split_once(": ") {
                    headers.insert(key.to_string().to_ascii_lowercase(), value.to_string());
                }
            } else {
                body.push_str(line);
            }
        }

        HttpResponse {
            status_code,
            body,
            headers,
        }
    }

    fn status_text(&self) -> &'static str {
        match self.status_code {
            200 => "OK",
            204 => "No Content",
            302 => "Found",
            304 => "Not Modified",
            _ => "Unknown Status",
        }
    }
}
