use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Request {
    pub method: String,
    pub path: String,
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

pub async fn start_server<F, Fut>(address: &str, handle_request: F)
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
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await.unwrap();

    let request_string = String::from_utf8_lossy(&buffer[..]);

    let request_lines: Vec<&str> = request_string.split("\r\n").collect();
    let request_line = request_lines[0];
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();

    let mut headers = Vec::new();
    for line in &request_lines[1..] {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.push((key.to_string(), value.to_string()));
        }
    }

    let request = Request {
        method,
        path,
        headers,
    };

    let response = handle_request(request).await; // Await the async handler

    let response_string = response.to_http_string();
    stream.write_all(response_string.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
