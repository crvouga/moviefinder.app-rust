#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::core::{
        http::{content_encoding::ContentEncoding, method::Method, request::Request},
        dynamic_data::DynamicData,
        url::{query_params::QueryParams, Url},
    };

    #[test]
    fn test_to_http_string_with_query_params() {
        let mut query_params = QueryParams::default();
        query_params.insert(&"key".to_string(), "value".to_string());

        let request = Request {
            url: Url {
                host: "example.com".to_string(),
                path: "/test".to_string(),
                query_params,
            },
            method: Method::Get,
            headers: Default::default(),
            cookies: Default::default(),
            form_data: Default::default(),
            body: Vec::new(),
        };

        let http_string = request.to_http_string();

        let expected =
            "GET /test?key=value HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        assert_eq!(http_string, expected);
    }

    #[test]
    fn test_to_http_string_with_custom_host_header() {
        let request = Request {
            url: Url {
                host: "example.com".to_string(),
                path: "/test".to_string(),
                query_params: Default::default(),
            },
            method: Method::Get,
            headers: HashMap::from([("Host".to_string(), "customhost.com".to_string())]),
            cookies: Default::default(),
            form_data: Default::default(),
            body: Vec::new(),
        };

        let http_string = request.to_http_string();

        let expected = "GET /test HTTP/1.1\r\nHost: customhost.com\r\nConnection: close\r\n\r\n";
        assert_eq!(http_string, expected);
    }

    #[test]
    fn test_to_accept_encoding() {
        let request = Request {
            url: Url {
                host: "example.com".to_string(),
                path: "/".to_string(),
                query_params: Default::default(),
            },
            method: Method::Get,
            headers: HashMap::from([("accept-encoding".to_string(), "gzip, deflate".to_string())]),
            cookies: Default::default(),
            form_data: Default::default(),
            body: Vec::new(),
        };

        let encodings = request.to_accept_encoding();

        assert_eq!(
            encodings,
            vec![ContentEncoding::Gzip, ContentEncoding::Deflate]
        );
    }

    #[test]
    fn test_to_http_string_with_body() {
        let request = Request {
            url: Url {
                host: "example.com".to_string(),
                path: "/submit".to_string(),
                query_params: Default::default(),
            },
            method: Method::Post,
            headers: HashMap::from([
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Content-Length".to_string(), "18".to_string()),
            ]),
            cookies: Default::default(),
            form_data: Default::default(),
            body: Vec::from(r#"{"key":"value"}"#),
        };

        let http_string = request.to_http_string();

        let expected = "POST /submit HTTP/1.1\r\nContent-Length: 18\r\nContent-Type: application/json\r\nHost: example.com\r\nConnection: close\r\n\r\n{\"key\":\"value\"}";
        assert_eq!(http_string, expected);
    }
}
