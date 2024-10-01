use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Request {
    pub method: String,
    pub path: String,
}

pub struct Response {
    pub status_code: u16,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, body: String) -> Response {
        Response { status_code, body }
    }

    pub fn to_http_string(&self) -> String {
        format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.body.len(),
            self.body
        )
    }
}

pub fn start_server(address: &str, handle_request: fn(Request) -> Response) {
    let listener = TcpListener::bind(address).unwrap();
    println!("Listening on {}", address);

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
