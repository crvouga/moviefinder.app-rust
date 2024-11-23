use super::form_data::FormData;
use super::method::HttpMethod;
use super::request::HttpRequest;
use super::response::HttpResponse;
use crate::core::params::Params;
use crate::core::url::query_params::QueryParams;
use crate::core::url::Url;
use crate::core::url_encoded;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn start<F, Fut>(address: &str, handle_request: F) -> Result<(), std::io::Error>
where
    F: Fn(HttpRequest) -> Fut + Send + Sync + 'static + Clone,
    Fut: std::future::Future<Output = HttpResponse> + Send + 'static,
{
    let bind = TcpListener::bind(address).await;

    let listener = match bind {
        Ok(listener) => listener,
        Err(err) => return Err(err),
    };

    loop {
        let accepted = listener.accept().await;
        let (stream, _) = match accepted {
            Ok(accepted) => accepted,
            Err(_) => continue,
        };
        let handle_request = handle_request.clone();
        tokio::spawn(handle_connection(stream, handle_request));
    }
}

async fn handle_connection<F, Fut>(mut stream: TcpStream, handle_request: F)
where
    F: Fn(HttpRequest) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = HttpResponse> + Send + 'static,
{
    let buffer = read_http_request(&mut stream).await;

    if buffer.is_empty() {
        return;
    }

    let headers_end = find_headers_end(&buffer).unwrap_or(0);

    let headers_data = &buffer[..headers_end];
    let request_string = parse_utf8_string(headers_data);
    let request_lines = split_request_lines(&request_string);

    let (method, path) = parse_request_line(request_lines[0]);
    let (headers, content_length) = parse_headers(&request_lines[1..]);

    let body = parse_body(&buffer, headers_end, content_length, &mut stream).await;
    let cookies = parse_cookies(&headers);
    let form_data = parse_form_data(&headers, &body);
    let query_params_string = path.split_once('?').map(|(_, query)| query).unwrap_or("");
    let query_params = QueryParams::from_string(query_params_string);
    let request = HttpRequest {
        method,
        url: Url {
            path,
            query_params,
            host: "".to_string(),
        },
        headers,
        cookies,
        body,
        form_data,
    };
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
    String::from_utf8_lossy(&data).to_string()
}

fn split_request_lines(request_string: &str) -> Vec<&str> {
    request_string.split("\r\n").collect()
}

fn parse_request_line(request_line: &str) -> (HttpMethod, String) {
    let mut parts = request_line.split_whitespace();
    let method = HttpMethod::from_string(parts.next().unwrap_or(""));
    let path = parts.next().unwrap_or("/").to_string();
    (method, path)
}

fn parse_cookies(headers: &HashMap<String, String>) -> HashMap<String, String> {
    let mut cookies = HashMap::new();

    if let Some(cookie_header) = headers.get("cookie") {
        for cookie in cookie_header.split("; ") {
            if let Some((key, value)) = cookie.split_once('=') {
                cookies.insert(key.to_string(), value.to_string());
            }
        }
    }

    cookies
}

fn parse_form_data(headers: &HashMap<String, String>, body: &Vec<u8>) -> FormData {
    if !is_form_data_request(headers) {
        return FormData::empty();
    }

    let body_string = String::from_utf8_lossy(&body).to_string();

    let decoded_body = url_encoded::decode(&body_string);

    FormData::from_string(&decoded_body)
}
fn is_form_data_request(headers: &HashMap<String, String>) -> bool {
    headers
        .get("content-type")
        .map(|content_type| content_type.starts_with("application/x-www-form-urlencoded"))
        .unwrap_or(false)
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
) -> Vec<u8> {
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

    body
}

async fn send_response(stream: &mut TcpStream, response: HttpResponse) {
    let response_bytes = response.to_http_bytes(); // Use `to_http_bytes` for binary-safe response
    if let Ok(()) = stream.write_all(&response_bytes).await {
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

    (0..=haystack.len() - needle.len()).find(|&i| haystack[i..i + needle.len()] == needle[..])
}
