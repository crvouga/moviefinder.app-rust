use super::{Request, Response};
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn start<F, Fut>(address: &str, handle_request: F)
where
    F: Fn(Request) -> Fut + Send + Sync + 'static + Clone,
    Fut: std::future::Future<Output = Response> + Send + 'static,
{
    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let handle_request = handle_request.clone();
        tokio::spawn(handle_connection(stream, handle_request));
    }
}

async fn handle_connection<F, Fut>(mut stream: TcpStream, handle_request: F)
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Response> + Send + 'static,
{
    let buffer = read_http_request(&mut stream).await;

    if buffer.is_empty() {
        return;
    }

    let headers_end = find_headers_end(&buffer).unwrap();

    let headers_data = &buffer[..headers_end];
    let request_string = parse_utf8_string(headers_data);
    let request_lines = split_request_lines(&request_string);

    let (method, path) = parse_request_line(&request_lines[0]);
    let (headers, content_length) = parse_headers(&request_lines[1..]);

    let body = parse_body(&buffer, headers_end, content_length, &mut stream).await;

    let request = new_request(method, path, headers, body);
    let response = handle_request(request).await;

    send_response(&mut stream, response).await;
}

async fn read_http_request(stream: &mut TcpStream) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];

    loop {
        match stream.read(&mut temp_buffer).await {
            Ok(0) => return Vec::new(),
            Ok(bytes_read) => {
                buffer.extend_from_slice(&temp_buffer[..bytes_read]);
                if find_subsequence(&buffer, b"\r\n\r\n").is_some() {
                    return buffer;
                }
            }
            Err(_) => return Vec::new(),
        }
    }
}

fn find_headers_end(buffer: &[u8]) -> Option<usize> {
    find_subsequence(buffer, b"\r\n\r\n")
}

fn parse_utf8_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}

fn split_request_lines(request_string: &str) -> Vec<&str> {
    request_string.split("\r\n").collect()
}

fn parse_request_line(request_line: &str) -> (String, String) {
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();
    (method, path)
}

fn parse_headers(header_lines: &[&str]) -> (HashMap<String, String>, usize) {
    let mut headers = HashMap::new();
    let mut content_length: usize = 0;

    for line in header_lines {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            if key.to_ascii_lowercase() == "content-length" {
                content_length = value.parse().unwrap_or(0);
            }
            headers.insert(key.to_string().to_ascii_lowercase(), value.to_string());
        }
    }

    (headers, content_length)
}

async fn parse_body(
    buffer: &[u8],
    headers_end: usize,
    content_length: usize,
    stream: &mut TcpStream,
) -> String {
    let mut body = Vec::new();

    let body_start = headers_end + 4;
    if body_start < buffer.len() {
        body.extend_from_slice(&buffer[body_start..]);
    }

    let remaining_bytes = content_length.saturating_sub(body.len());
    if remaining_bytes > 0 {
        let mut remaining_body = vec![0; remaining_bytes];
        if stream.read_exact(&mut remaining_body).await.is_ok() {
            body.extend_from_slice(&remaining_body);
        }
    }

    String::from_utf8_lossy(&body).to_string()
}

fn new_request(
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
) -> Request {
    Request {
        method,
        path,
        host: "".to_string(),
        headers,
        body,
    }
}

async fn send_response(stream: &mut TcpStream, response: Response) {
    let response_string = response.to_http_string();
    if let Ok(()) = stream.write_all(response_string.as_bytes()).await {
        let _ = stream.flush().await;
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    if needle.len() > haystack.len() {
        return None;
    }

    for i in 0..=haystack.len() - needle.len() {
        if haystack[i..i + needle.len()] == needle[..] {
            return Some(i);
        }
    }
    None
}
