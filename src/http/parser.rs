pub use crate::http::UriPath;
pub use crate::http::{HttpMethod, HttpRequest, HttpVersion, RequestLine};

use winnow::prelude::*;
use winnow::token::{take_until0, take_while};
use winnow::{ascii::line_ending, combinator::repeat};

use std::collections::HashMap;

// ---- -- HTTP Version -- ---- \\

/// Parsea la versión HTTP en el formato "n.n" desde la entrada proporcionada.
fn http_version(input: &mut &[u8]) -> PResult<HttpVersion> {
    let _ = "HTTP/".parse_next(input)?;
    let version = take_while(1.., is_version).parse_next(input)?;
    let version = std::str::from_utf8(version).unwrap();
    let version = HttpVersion::from(version);

    Ok(version)
}

// ---- -- HTTP Method -- ---- \\

fn http_method(input: &mut &[u8]) -> PResult<HttpMethod> {
    let method = take_while(1.., is_token).parse_next(input)?;
    let method = std::str::from_utf8(method).unwrap();
    let method = HttpMethod::from(method);

    Ok(method)
}

// ---- -- URI path -- ---- \\

/// Parsea la ruta URI desde la entrada proporcionada.
fn http_uri(input: &mut &[u8]) -> PResult<UriPath> {
    let uri = take_while(1.., is_not_space).parse_next(input)?;
    let uri = std::str::from_utf8(uri).unwrap();
    let uri = UriPath::new(uri);

    Ok(uri)
}

// ---- -- HTTP Request line -- ---- \\

fn request_line(input: &mut &[u8]) -> PResult<RequestLine> {
    let method = http_method(input)?;
    let _ = take_while(1.., is_space).parse_next(input)?;
    let uri = http_uri(input)?;
    let _ = take_while(1.., is_space).parse_next(input)?;
    let http_version = http_version(input)?;
    let _ = line_ending.parse_next(input)?;

    Ok(RequestLine {
        method,
        uri,
        http_version,
    })
}

// ---- -- HTTP Headers -- ---- \\

fn message_header_value<'i>(input: &mut &'i [u8]) -> PResult<&'i str> {
    let _ = take_while(1.., is_horizontal_space).parse_next(input)?;
    let data = take_while(1.., not_line_ending).parse_next(input)?;

    if !input.is_empty() && &input[0..2] == b"\r\n" {
        let _ = line_ending.parse_next(input)?;
    }

    let data = std::str::from_utf8(data).unwrap();
    Ok(data)
}

fn message_header(input: &mut &[u8]) -> PResult<(String, String)> {
    let name = take_while(1.., is_token).parse_next(input)?;
    let name = std::str::from_utf8(name).unwrap();

    let _ = ':'.parse_next(input)?;
    let value = repeat(1.., message_header_value).parse_next(input)?;

    Ok((name.to_string(), value))
}

// ---- -- Parse Request metadata -- ---- \\

/// Parsea la información de la solicitud HTTP,
/// incluyendo la línea de solicitud y los headers.
fn parse_request_metadata(
    input: &mut &[u8],
) -> PResult<(RequestLine, HashMap<String, String>)> {
    let request_line = request_line(input)?;

    let mut headers_hash: HashMap<String, String> = HashMap::new();

    let headers: Vec<(String, String)> =
        repeat(1.., message_header).parse_next(input)?;
    headers.iter().for_each(|(key, value)| {
        headers_hash.insert(key.clone(), value.clone());
    });

    Ok((request_line, headers_hash))
}

// ---- -- Parse Request -- ---- \\

/// Parsea la solicitud HTTP completa, incluyendo
/// la línea de solicitud, los headers y el cuerpo.
pub fn parse_request(input: &mut &[u8]) -> PResult<HttpRequest> {
    // Encuentra la posición del patrón '\r\n\r\n',
    // en el input para separar request y body.
    let mut line_blank = "\r\n\r\n";

    let mut request = take_until0(line_blank).parse_next(input)?;
    let (request_line, headers) = parse_request_metadata(&mut request).unwrap();

    // Eliminar cualquier espacio en blanco o saltos de línea adicionales
    // después del patrón \r\n\r\n.
    let _ = line_blank.parse_next(input)?;

    consume_whitespace(input);
    let body = if input.is_empty() {
        None
    } else {
        Some(input.to_vec())
    };

    Ok(HttpRequest {
        request_line,
        headers,
        body,
    })
}

fn consume_whitespace(input: &mut &[u8]) {
    while let Some(&b) = input.first() {
        if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
            *input = &input[1..];
        } else {
            break;
        }
    }
}

// ---- -- Validators -- ---- \\

/// Verifica si un byte es un carácter ASCII dígito o un punto.
/// Utilizado para validar sentencias en el formato "n.n".
fn is_version(c: u8) -> bool {
    c.is_ascii_digit() || c == b'.'
}

fn is_space(c: u8) -> bool {
    c == b' '
}

fn is_horizontal_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn not_line_ending(c: u8) -> bool {
    c != b'\r' && c != b'\n'
}

fn is_not_space(c: u8) -> bool {
    c != b' '
}

#[rustfmt::skip]
#[allow(clippy::match_same_arms)]
#[allow(clippy::match_like_matches_macro)]
fn is_token(c: u8) -> bool {
  match c {
    128..=255 => false,
    0..=31    => false,
    b'('      => false,
    b')'      => false,
    b'<'      => false,
    b'>'      => false,
    b'@'      => false,
    b','      => false,
    b';'      => false,
    b':'      => false,
    b'\\'     => false,
    b'"'      => false,
    b'/'      => false,
    b'['      => false,
    b']'      => false,
    b'?'      => false,
    b'='      => false,
    b'{'      => false,
    b'}'      => false,
    b' '      => false,
    _         => true,
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- HTTP Version -- ---- \\

    #[test]
    fn test_http_version_parsing() {
        let mut input: &[u8] = b"HTTP/1.1";
        let expected_version = HttpVersion::V1_1;

        let version = http_version(&mut input).unwrap();

        assert_eq!(expected_version, version);
    }

    // ---- -- Request Line -- ---- \\

    #[test]
    fn test_request_line_parsing_get_method() {
        let mut input: &[u8] = b"GET /data/test.html HTTP/1.1\n";
        let expected_request_line = RequestLine {
            method: HttpMethod::Get,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };

        let request_line = request_line(&mut input).unwrap();

        assert_eq!(expected_request_line, request_line);
    }

    #[test]
    fn test_request_line_parsing_post_method() {
        let mut input: &[u8] = b"POST /files/test.html HTTP/1.1\n";
        let expected_request_line = RequestLine {
            method: HttpMethod::Post,
            uri: UriPath::new("/files/test.html"),
            http_version: HttpVersion::V1_1,
        };

        let request_line = request_line(&mut input).unwrap();

        assert_eq!(expected_request_line, request_line);
    }

    // ---- -- HttpMethod -- ---- \\

    #[test]
    fn test_http_method_parsing() {
        let mut input: &[u8] = b"GET /data/test.html HTTP/1.1";
        let expected_method = HttpMethod::Get;

        let method = http_method(&mut input).unwrap();

        assert_eq!(expected_method, method);
    }

    // ---- -- HTTP Headers -- ---- \\

    #[test]
    fn test_http_header_parsing() {
        let mut input: &[u8] = b"Host: www.test101.com\r\n";

        let (key, value) = message_header(&mut input).unwrap();

        assert_eq!("Host".to_string(), key);
        assert_eq!("www.test101.com".to_string(), value);
    }

    // ---- -- Parse HTTP Request -- ---- \\

    #[test]
    fn test_request_metadata_parsing() {
        let expected_request_line = RequestLine {
            method: HttpMethod::Get,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };
        let request_lines = [
            "GET /data/test.html HTTP/1.1",
            "Host: www.test101.com",
            "Accept: image/gif, image/jpeg, *//*",
            "Accept-Language: en-us",
            "Accept-Encoding: gzip, deflate",
            "User-Agent: Mozilla/4.0",
            "Content-Length: 35",
        ];
        let plain_headers: String = request_lines.join("\r\n");

        let (request_line, headers) =
            parse_request_metadata(&mut plain_headers.as_bytes()).unwrap();

        assert_eq!(expected_request_line, request_line);
        for (key, value) in headers.iter() {
            assert!(plain_headers.contains(key));
            assert!(plain_headers.contains(value));
        }
    }

    #[test]
    fn test_complex_request_with_body_parsing() {
        let request_lines = [
            "GET /data/test.html HTTP/1.1",
            "Host: www.test101.com",
            "Accept: image/gif, image/jpeg, *//*",
            "Accept-Language: en-us",
            "Accept-Encoding: gzip, deflate",
            "Content-Length: 35",
            "User-Agent: Mozilla/4.0",
            "",
            "bookId=12345&author=Tan+Ah+Teck",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "www.test101.com".into());
        headers_expected
            .insert("Accept".into(), "image/gif, image/jpeg, *//*".into());
        headers_expected.insert("Accept-Language".into(), "en-us".into());
        headers_expected
            .insert("Accept-Encoding".into(), "gzip, deflate".into());
        headers_expected.insert("User-Agent".into(), "Mozilla/4.0".into());
        headers_expected.insert("Content-Length".into(), "35".into());

        // Ordenar los elementos del mapa headers_expected
        let mut sorted_headers_expected: Vec<_> =
            headers_expected.into_iter().collect();
        sorted_headers_expected.sort();

        let expect_request_line = RequestLine {
            method: HttpMethod::Get,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };
        let expect_body = request_lines[8].as_bytes();

        let request = parse_request(&mut plain_request.as_bytes()).unwrap();

        assert_eq!(expect_request_line, request.request_line);

        // Obtener los headers del request
        let mut request_headers: Vec<_> = request.headers.into_iter().collect();
        request_headers.sort();
        assert_eq!(sorted_headers_expected, request_headers);

        assert_eq!(expect_body, request.body.unwrap());
    }

    #[test]
    fn test_complex_request_without_body_parsing() {
        let request_lines = [
            "POST /data/test.html HTTP/1.1",
            "Host: www.test101.com",
            "Accept-Language: en-us",
            "Accept-Encoding: gzip, deflate",
            "User-Agent: Mozilla/4.0",
            "Content-Length: 0",
            "\r\n",
            "",
        ];
        let plain_request: String = request_lines.join("\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "www.test101.com".into());
        headers_expected.insert("Accept-Language".into(), "en-us".into());
        headers_expected
            .insert("Accept-Encoding".into(), "gzip, deflate".into());
        headers_expected.insert("User-Agent".into(), "Mozilla/4.0".into());
        headers_expected.insert("Content-Length".into(), "0".into());

        // Ordenar los elementos del mapa headers_expected
        let mut sorted_headers_expected: Vec<_> =
            headers_expected.into_iter().collect();
        sorted_headers_expected.sort();

        let expect_request_line = RequestLine {
            method: HttpMethod::Post,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };

        let request = parse_request(&mut plain_request.as_bytes()).unwrap();

        assert_eq!(expect_request_line, request.request_line);

        // Obtener los headers del request
        let mut request_headers: Vec<_> = request.headers.into_iter().collect();
        request_headers.sort();
        assert_eq!(sorted_headers_expected, request_headers);

        assert!(request.body.is_none());
    }
}
