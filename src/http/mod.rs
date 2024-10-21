pub mod client;
pub mod server;

pub struct Request {
    pub method: String,
    pub path: String,
    pub host: String, // Add host field directly in the Request struct
    pub headers: Vec<(String, String)>,
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
}
