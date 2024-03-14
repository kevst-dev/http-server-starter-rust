use crate::http::{parse_request, UriPath};

// ---- -- HTTP Method -- ---- \\

// Representa el método HTTP de una solicitud.
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    // Método no inicializado o desconocido
    Uninitialized,
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> HttpMethod {
        match s {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => HttpMethod::Uninitialized,
        }
    }
}

// ---- -- HTTP Version -- ---- \\

// Representa la versión de protocolo HTTP de una solicitud.
#[derive(Debug, Clone, PartialEq)]
pub enum HttpVersion {
    V1_1,
    // Version no inicializada o desconocida
    Uninitialized,
}

impl From<&str> for HttpVersion {
    fn from(s: &str) -> HttpVersion {
        match s {
            "1.1" => HttpVersion::V1_1,
            _ => HttpVersion::Uninitialized,
        }
    }
}

// ---- -- Request Line -- ---- \\

// Representa los atributos de la cabecera de una solicitud HTTP.
#[derive(Debug, Clone, PartialEq)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub uri: UriPath,
    pub http_version: HttpVersion,
}

impl RequestLine {
    pub fn new(method: &str, uri: &str, http_version: &str) -> RequestLine {
        RequestLine {
            method: HttpMethod::from(method),
            uri: UriPath::new(uri),
            http_version: HttpVersion::from(http_version),
        }
    }
}

// ---- -- Http Request -- ---- \\

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn uri(&self) -> &UriPath {
        &self.request_line.uri
    }
    pub fn method(&self) -> &HttpMethod {
        &self.request_line.method
    }
    pub fn version(&self) -> &HttpVersion {
        &self.request_line.http_version
    }
}

impl From<&[u8]> for HttpRequest {
    fn from(request: &[u8]) -> Self {
        let mut request = request;

        parse_request(&mut request).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- HttpMethod -- ---- \\

    #[test]
    fn test_method_into() {
        let m: HttpMethod = "GET".into();

        assert_eq!(m, HttpMethod::Get);
    }

    // ---- -- HttpVersion -- ---- \\

    #[test]
    fn test_version_into() {
        let m: HttpVersion = "1.1".into();

        assert_eq!(m, HttpVersion::V1_1);
    }

    // ---- -- HttpRequest -- ---- \\

    // GET

    #[test]
    #[ignore]
    fn test_read_http_get_empty_request() {
        let plain_request: String = String::from("GET / HTTP/1.1\n \r\n\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Get, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/", request.uri().to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_get_with_headers() {
        let request_lines = [
            "GET /greeting HTTP/1.1",
            "Host: localhost:3000",
            "User-Agent: curl/7.64.1",
            "Accept: */*",
            "\r\n",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost:3000".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());

        // Ordenar los elementos del mapa headers_expected
        let mut sorted_headers_expected: Vec<_> =
            headers_expected.into_iter().collect();
        sorted_headers_expected.sort();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Get, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/greeting", request.uri().to_string());

        // Obtener los headers del request
        let mut request_headers: Vec<_> = request.headers.into_iter().collect();
        request_headers.sort();
        assert_eq!(sorted_headers_expected, request_headers);

        assert!(request.body.is_none());
    }

    #[ignore]
    #[test]
    fn test_read_http_get_without_headers() {
        let request_lines = ["GET /api/data HTTP/1.1", "\r\n"];
        let plain_request: String = request_lines.join("\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Get, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/api/data", request.uri().to_string());

        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_get_with_body() {
        let request_lines = [
            "GET /data HTTP/1.1",
            "Host: example.com",
            "Content-Length: 15",
            "\r\n",
            "Hello, World!",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "example.com".into());
        headers_expected.insert("Content-Length".into(), "15".into());

        // Ordenar los elementos del mapa headers_expected
        let mut sorted_headers_expected: Vec<_> =
            headers_expected.into_iter().collect();
        sorted_headers_expected.sort();

        let request = HttpRequest::from(plain_request.as_bytes());

        println!("{:?}", request);

        assert_eq!(HttpMethod::Get, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/data", request.uri().to_string());

        // Obtener los headers del request
        let mut request_headers: Vec<_> = request.headers.into_iter().collect();
        request_headers.sort();
        assert_eq!(sorted_headers_expected, request_headers);

        let body = String::from_utf8(request.body.unwrap()).unwrap();
        assert_eq!("Hello, World!", body);
    }

    // POST

    #[ignore]
    #[test]
    fn test_read_http_post_empty_request() {
        let plain_request: String = String::from("POST / HTTP/1.1\r\n\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Post, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/", request.uri().to_string());
        assert_eq!(headers_expected, request.headers);

        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_post_with_headers() {
        let request_lines = [
            "POST /greeting HTTP/1.1",
            "Host: 197.0.0.1:3000",
            "User-Agent: curl/7.64.1",
            "Accept: */*",
            "\r\n",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "197.0.0.1:3000".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());

        // Ordenar los elementos del mapa headers_expected
        let mut sorted_headers_expected: Vec<_> =
            headers_expected.into_iter().collect();
        sorted_headers_expected.sort();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Post, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/greeting", request.uri().to_string());

        // Obtener los headers del request
        let mut request_headers: Vec<_> = request.headers.into_iter().collect();
        request_headers.sort();
        assert_eq!(sorted_headers_expected, request_headers);

        assert!(request.body.is_none());
    }

    #[test]
    #[ignore]
    fn test_read_http_post_without_headers() {
        let request_lines = ["POST /api/data HTTP/1.1", "\r\n"];
        let plain_request: String = request_lines.join("\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Post, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/api/data", request.uri().to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_post_with_body() {
        let request_lines = [
            "POST /data HTTP/1.1",
            "Host: example.com",
            "Content-Length: 15",
            "\r\n",
            "Hello, World!",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "example.com".into());
        headers_expected.insert("Content-Length".into(), "15".into());

        // Ordenar los elementos del mapa headers_expected
        let mut sorted_headers_expected: Vec<_> =
            headers_expected.into_iter().collect();
        sorted_headers_expected.sort();

        let request: HttpRequest = plain_request.as_bytes().into();

        assert_eq!(HttpMethod::Post, request.method().clone());
        assert_eq!(HttpVersion::V1_1, request.version().clone());
        assert_eq!("/data", request.uri().to_string());

        // Obtener los headers del request
        let mut request_headers: Vec<_> = request.headers.into_iter().collect();
        request_headers.sort();
        assert_eq!(sorted_headers_expected, request_headers);

        let body = String::from_utf8(request.body.unwrap()).unwrap();
        assert_eq!("Hello, World!", body);
    }
}
