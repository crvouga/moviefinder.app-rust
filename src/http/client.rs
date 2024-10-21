use super::{Request, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send(request: Request) -> tokio::io::Result<Response> {
    let addr = format!("{}:80", request.host);
    let mut stream = TcpStream::connect(addr).await?;

    let request_string = request.to_http_string();
    stream.write_all(request_string.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let response_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

    let response = Response::from_http_string(&response_string);

    Ok(response)
}
