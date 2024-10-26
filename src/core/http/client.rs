use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use super::request::HttpRequest;
use super::response::HttpResponse;

pub async fn send(request: HttpRequest) -> tokio::io::Result<HttpResponse> {
    let addr = format!("{}:80", request.host);
    let mut stream = TcpStream::connect(addr).await?;

    let request_string = request.to_http_string();
    stream.write_all(request_string.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let response_string: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

    let response = HttpResponse::from_http_string(&response_string);

    println!(
        "LOG Http Request:\n\t{:?}\n\t{:?}",
        request.path, request.query_params,
    );

    println!(
        "LOG Http Response:\n\t{:?}\n\t{:?}",
        response.status_code,
        response.body.chars().take(1000).collect::<String>(),
    );

    Ok(response)
}
