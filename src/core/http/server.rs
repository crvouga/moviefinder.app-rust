use std::collections::HashMap;

use super::{Request, Response};
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
        println!("Connection established");
        let handle_request = handle_request.clone();
        tokio::spawn(handle_connection(stream, handle_request));
    }
}

async fn handle_connection<F, Fut>(mut stream: TcpStream, handle_request: F)
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Response> + Send + 'static,
{
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    let request_string = String::from_utf8_lossy(&buffer[..]);

    let request_lines: Vec<&str> = request_string.split("\r\n").collect();
    let request_line = request_lines[0];
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();

    let mut headers = HashMap::new();
    for line in &request_lines[1..] {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string().to_ascii_lowercase(), value.to_string());
        }
    }

    let request = Request {
        method,
        path,
        host: "".to_string(), // Server does not need the host field
        headers,
    };

    let response = handle_request(request).await; // Await the async handler

    let response_string = response.to_http_string();
    stream.write_all(response_string.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
