use std::collections::HashMap;

use crate::core::res::Res;

pub mod client;
pub mod server;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn to_http_string(&self) -> String {
        let mut headers_string = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}\r\n", key, value))
            .collect::<String>();

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

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status_code: u16, body: String, headers: HashMap<String, String>) -> Response {
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

    fn from_http_string(response: &str) -> Self {
        let mut lines = response.lines();
        let status_line = lines.next().unwrap();
        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .unwrap()
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

        Response {
            status_code,
            body,
            headers,
        }
    }
}

impl From<Res> for Response {
    fn from(res: Res) -> Self {
        match res {
            Res::Html(body) => Response::new(200, body.render(), HashMap::new()),
            Res::Redirect(location) => {
                let mut headers = HashMap::new();
                headers.insert(
                    "Location".to_string().to_ascii_lowercase(),
                    ensure_leading_slash(&location),
                );
                Response::new(302, "".to_owned(), headers)
            }
            Res::Empty => Response::new(204, "".to_owned(), HashMap::new()),
        }
    }
}

fn ensure_leading_slash(path: &str) -> String {
    if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{}", path)
    }
}
