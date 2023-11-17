use crate::url_path::UrlPath;

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
            "HTTP/1.1" => HttpVersion::V1_1,
            _ => HttpVersion::Uninitialized,
        }
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub version: HttpVersion,
    pub resource: UrlPath,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

fn process_req_line(line_request: &str) -> (HttpMethod, UrlPath, HttpVersion) {
    // Divide la línea de solicitud en palabras separadas por espacios
    let mut words = line_request.split_whitespace();

    // HTTP de la primera parte de la línea de solicitud
    let method = words.next().unwrap();

    // (URI/URL) de la segunda parte de la línea de solicitud
    let resource = words.next().unwrap();

    // Version HTTP de la tercera parte de la línea de solicitud
    let version = words.next().unwrap();

    (
        HttpMethod::from(method.trim()),
        UrlPath::new(resource),
        HttpVersion::from(version.trim()),
    )
}

fn process_header_line(line_request: &str) -> (String, String) {
    // Divide el encabezado en palabras separadas por el separador (':')
    let mut key = String::from("");
    let mut value = String::from("");

    let mut header_items = line_request.split(':');

    // Extrae la clave del encabezado
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    // Extrae el valor del encabezado
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }

    (key, value)
}

impl From<String> for HttpRequest {
    fn from(request: String) -> Self {
        println!("{}", request);

        let mut parsed_method = HttpMethod::Uninitialized;
        let mut parsed_version = HttpVersion::V1_1;
        let mut parsed_resource = UrlPath::new("");
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = Vec::new();
        let mut is_body = false;

        // Lee cada línea en la solicitud HTTP entrante
        for line in request.lines() {
            match line {
                // Si es una línea de solicitud
                line if line.contains("HTTP") => {
                    (parsed_method, parsed_resource, parsed_version) =
                        process_req_line(line);
                }
                // Si es una línea de encabezado
                line if line.contains(':') && !is_body => {
                    let (key, value) = process_header_line(line);
                    parsed_headers.insert(key, value);
                }
                // Si es una línea en blanco: no haggis nada
                line if line.is_empty() => {
                    is_body = true;
                },
                // Si no tiene coincidencia: es el cuerpo del mensaje
                _ if is_body => {
                    parsed_body.append(&mut line.as_bytes().to_vec());
                    parsed_body.append(&mut "\n".as_bytes().to_vec());
                }
                _ => (),
            }
        }

        let parsed_body = if parsed_body.last().is_none() {
            None
        } else {
            Some(parsed_body)
        };

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body,
        }
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
        let m: HttpVersion = "HTTP/1.1".into();

        assert_eq!(m, HttpVersion::V1_1);
    }

    // ---- -- HttpRequest -- ---- \\

    // GET

    #[test]
    fn test_read_http_get_empty_request() {
        let plain_request: String = String::from("GET / HTTP/1.1\r\n\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Get, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/", request.resource.to_string());
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
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Get, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/greeting", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_get_without_headers() {
        let request_lines = ["GET /api/data HTTP/1.1", "\r\n"];
        let plain_request: String = request_lines.join("\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Get, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/api/data", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_get_with_body() {
        let request_lines = [
            "GET /data HTTP/1.1",
            "Host: example.com",
            "Content-Length: 15",
            "Hello, World!",
            "\r\n",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " example.com".into());
        headers_expected.insert("Content-Length".into(), " 15".into());

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Get, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/data", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);

        let body = String::from_utf8(request.body.unwrap()).unwrap();
        assert_eq!("Hello, World!", body);
    }

    // POST

    #[test]
    fn test_read_http_post_empty_request() {
        let plain_request: String = String::from("POST / HTTP/1.1\r\n\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Post, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_post_with_headers() {
        let request_lines = [
            "POST /greeting HTTP/1.1",
            "Host: localhost:3000",
            "User-Agent: curl/7.64.1",
            "Accept: */*",
            "\r\n",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Post, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/greeting", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_post_without_headers() {
        let request_lines = ["POST /api/data HTTP/1.1", "\r\n"];
        let plain_request: String = request_lines.join("\r\n");

        let headers_expected = HashMap::new();

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Post, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/api/data", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);
        assert!(request.body.is_none());
    }

    #[test]
    fn test_read_http_post_with_body() {
        let request_lines = [
            "POST /data HTTP/1.1",
            "Host: example.com",
            "Content-Length: 15",
            "Hello, World!",
            "\r\n",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " example.com".into());
        headers_expected.insert("Content-Length".into(), " 15".into());

        let request: HttpRequest = plain_request.into();

        assert_eq!(HttpMethod::Post, request.method);
        assert_eq!(HttpVersion::V1_1, request.version);
        assert_eq!("/data", request.resource.to_string());
        assert_eq!(headers_expected, request.headers);
        assert_eq!(headers_expected, request.headers);

        let body = String::from_utf8(request.body.unwrap()).unwrap();
        assert_eq!("Hello, World!", body);
    }
}
