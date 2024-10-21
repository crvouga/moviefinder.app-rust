use super::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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

impl Response {
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

pub async fn send(request: Request) -> tokio::io::Result<Response> {
    let addr = format!("{}:80", request.host); // Use host from the Request struct
    let mut stream = TcpStream::connect(addr).await?;

    let request_string = request.to_http_string();
    stream.write_all(request_string.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let response_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

    let response = Response::from_http_response(&response_string);

    Ok(response)
}
