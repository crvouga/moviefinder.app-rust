pub mod client;
pub mod server;

pub struct Request {
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    pub fn to_http_string(&self) -> String {
        let headers_string = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}\r\n", key, value))
            .collect::<String>();

        format!(
            "{} {} HTTP/1.1\r\n{}Connection: close\r\n\r\n",
            self.method, self.path, headers_string
        )
    }
}

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
    pub headers: Vec<(String, String)>,
}

impl Response {
    pub fn new(status_code: u16, body: String, headers: Vec<(String, String)>) -> Response {
        Response {
            status_code,
            body,
            headers,
        }
    }

    pub fn to_http_string(&self) -> String {
        let headers_string = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}\r\n", key, value))
            .collect::<String>();

        format!(
            "HTTP/1.1 {} OK\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status_code,
            headers_string,
            self.body.len(),
            self.body
        )
    }

    fn from_http_response(response: &str) -> Self {
        let mut lines = response.lines();
        let status_line = lines.next().unwrap();
        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u16>()
            .unwrap_or(500);

        let mut headers = Vec::new();
        let mut body = String::new();
        let mut in_headers = true;

        for line in lines {
            if line.is_empty() {
                in_headers = false;
            } else if in_headers {
                if let Some((key, value)) = line.split_once(": ") {
                    headers.push((key.to_string(), value.to_string()));
                }
            } else {
                body.push_str(line);
            }
        }

        Response {
            status_code,
            body,
            headers,
        }
    }
}
