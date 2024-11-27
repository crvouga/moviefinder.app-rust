static MIME_MAP: &[(&str, &str)] = &[
    ("html", "text/html"),
    ("htm", "text/html"),
    ("css", "text/css"),
    ("js", "application/javascript"),
    ("json", "application/json"),
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("svg", "image/svg+xml"),
    ("ico", "image/x-icon"),
    ("txt", "text/plain"),
    ("pdf", "application/pdf"),
    ("xml", "application/xml"),
    ("zip", "application/zip"),
    ("rar", "application/x-rar-compressed"),
    ("mp4", "video/mp4"),
    ("mp3", "audio/mpeg"),
    ("wav", "audio/wav"),
    ("ogg", "audio/ogg"),
    ("webm", "video/webm"),
    ("woff", "font/woff"),
    ("woff2", "font/woff2"),
    ("ttf", "font/ttf"),
    ("otf", "font/otf"),
];

pub fn to_mime_type(extension: &str) -> &str {
    MIME_MAP
        .iter()
        .find(|&&(ext, _)| ext == extension)
        .map(|&(_, mime)| mime)
        .unwrap_or("application/octet-stream")
}

pub fn to_extension(file_path: &str) -> &str {
    let extension = file_path.split('.').last().unwrap_or("");
    extension
}

pub fn from_path(file_path: &str) -> &str {
    let extension = to_extension(file_path);
    let mime_type = to_mime_type(extension);
    mime_type
}
