pub fn to_reason(status_code: u16) -> String {
    let reason = match status_code {
        200 => "OK",
        301 => "Moved Permanently",
        302 => "Found",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };

    reason.to_string()
}
