use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Request {
    pub method: String,
    pub path: String,
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

pub fn start_server(address: &str, handle_request: fn(Request) -> Response) {
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, handle_request);
    }
}

fn handle_connection(mut stream: TcpStream, handle_request: fn(Request) -> Response) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request_string = String::from_utf8_lossy(&buffer[..]);

    let request_lines: Vec<&str> = request_string.split("\r\n").collect();
    let request_line = request_lines[0];
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();

    let request = Request { method, path };

    let response = handle_request(request);

    let response_string = response.to_http_string();
    stream.write_all(response_string.as_bytes()).unwrap();
    stream.flush().unwrap();
}
