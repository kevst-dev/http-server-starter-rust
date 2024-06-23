pub use crate::http::UriPath;
pub use crate::http::{HttpMethod, HttpRequest, HttpVersion, RequestLine};

use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_while1, take_until},
    character::{is_alphanumeric, is_hex_digit, is_space},
    combinator::{map, map_res, opt},
    multi::many0,
    IResult,
};
use anyhow;

use std::str;
use std::str::FromStr;
use std::collections::HashMap;

// ---- -- HTTP Version -- ---- \\

/// Parsea la versión HTTP en el formato "n.n" desde la entrada proporcionada.
fn http_version(input: &[u8]) -> IResult<&[u8], HttpVersion> {
    // Valida si una sentencia en del tipo "n.n".
    let is_version = |c| c >= b'0' && c <= b'9' || c == b'.';

    let (input, _) = tag("HTTP/")(input)?;
    let (input, version) = take_while1(is_version).parse(input)?;

    let version = std::str::from_utf8(version).unwrap();
    let version = HttpVersion::from(version);

    Ok((input, version))
}

// ---- -- HTTP Method -- ---- \\

fn http_method(input: &[u8]) -> IResult<&[u8], HttpMethod> {
    let mut method = take_while1(is_alphanumeric);

    let (input, method) = method.parse(input)?;
    let method = std::str::from_utf8(method).unwrap();
    let method = HttpMethod::from(method);

    Ok((input, method))
}

// ---- -- URI path -- ---- \\

/// Parsea la ruta URI desde la entrada proporcionada.
fn http_uri(input: &[u8]) -> IResult<&[u8],UriPath> {
    let mut is_not_space = take_while1(|c| c != b' ');

    let (input, uri)= is_not_space.parse(input)?;
    let uri = std::str::from_utf8(uri).unwrap();
    let uri = UriPath::new(uri);

    Ok((input, uri))
}

// ---- -- HTTP Request line -- ---- \\

fn request_line(input: &[u8]) -> IResult<&[u8], RequestLine> {
    let mut space = take_while1(|c| c == b' ');
    let line_ending = alt( (tag("\r\n"), tag("\n")) );

    let (input, method) = http_method(input)?;
    let (input, _) = space.parse(input)?;
    let (input, uri) = http_uri(input)?;
    let (input, _) = space.parse(input)?;
    let (input, http_version) = http_version(input)?;

    // Es posile que haya un "\r\n" al final o que no
    let (input, _) = opt(line_ending)(input)?;

    let request_line = RequestLine {
        method,
        uri,
        http_version,
    };

    Ok((input, request_line))
}

// ---- -- HTTP Headers -- ---- \\

fn header(input: &[u8]) -> IResult< &[u8], (String, String) > {
    let mut is_key = take_while1(|c| c != b':');
    let mut is_token = take_while1(|c| c != b'\r' && c != b'\n');
    let mut line_ending = alt( (tag("\r\n"), tag("\n")) );

    let (input, key) = is_key.parse(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = is_token.parse(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let key = std::str::from_utf8(key).unwrap();
    let value = std::str::from_utf8(value).unwrap();

    let result = (key.to_string(), value.to_string());
    Ok((input, result))
}

// ---- -- Parse Request metadata -- ---- \\

/// Parsea la información de la solicitud HTTP,
/// incluyendo la línea de solicitud y los headers.
fn parse_request_metadata(input: &[u8]) -> IResult<&[u8], (RequestLine, HashMap<String, String>)> {
    let mut headers_hash: HashMap<String, String> = HashMap::new();

    let (input, request_line) = request_line(input)?;
    let (input, headers) = opt(many0(header))(input)?;

    if let Some(headers) = headers {
        headers.iter().for_each(|(key, value)| {
            headers_hash.insert(key.clone(), value.clone());
        });
    }

    let result = (request_line, headers_hash);
    Ok((input, result))
}

// ---- -- Parse Request -- ---- \\

/// Parsea la solicitud HTTP completa, incluyendo
/// la línea de solicitud, los headers y el cuerpo.
fn request(input: &[u8]) -> IResult<&[u8], HttpRequest> {
    // Encuentra la posición del patrón '\r\n\r\n',
    // en el input para separar request y body.
    let mut line_blank = "\r\n\r\n";

    let (input, request) = take_until(line_blank)(input)?;
    let (_, metadata) = parse_request_metadata(request)?;
    let (request_line, headers) = metadata;
    let (input, _) = opt(tag(line_blank))(input)?;
    let (mut input, _) = opt(tag("\r\n"))(input)?;

    let body = if input.is_empty() {
        None
    } else {
        Some(input.to_vec())
    };

    if body.is_some() {
        input = &b""[..]; // Limpiar el input
    }

    let result = HttpRequest {
        request_line,
        headers,
        body,
    };

    Ok((input, result))
}

pub fn parse_request(input: &[u8]) -> anyhow::Result<HttpRequest> {
    let parse_result = request(input);

    parse_result
        .map(|(_, input)| input)
        .map_err(|err| match err {
            nom::Err::Incomplete(needed) => {
                let message = format!("Incomplete input: {:?}", needed);
                anyhow::anyhow!(message)
            }
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                let message = format!("Failed to parse input: {:?}", e);
                anyhow::anyhow!(message)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- HTTP Version -- ---- \\

    #[test]
    fn test_http_version_parsing() {
        let input: &[u8] = b"HTTP/1.1";

        let result = http_version(input).unwrap();
        let expected = (&b""[..], HttpVersion::V1_1);

        assert_eq!(result, expected);
    }

    // ---- -- HttpMethod -- ---- \\

    #[test]
    fn test_http_method_parsing() {
        let mut input: &[u8] = b"GET /data/test.html HTTP/1.1";
        let expected_method = HttpMethod::Get;

        let (input, method) = http_method(&mut input).unwrap();
        assert_eq!(expected_method, method);

        let expected_input = b" /data/test.html HTTP/1.1";
        assert_eq!(input, expected_input);
    }

    // ---- -- URI path -- ---- \\

    #[test]
    fn test_http_uri_parsing() {
        let mut input: &[u8] = b"/data/test.html";
        let expected_uri = UriPath::new("/data/test.html");

        let (input, uri) = http_uri(&mut input).unwrap();
        assert_eq!(expected_uri, uri);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    // ---- -- Request Line -- ---- \\

    #[test]
    fn test_request_line_parsing_get_method_with_line_ending() {
        let mut input: &[u8] = b"GET /data/test.html HTTP/1.1\r\n";

        let expected_request_line = RequestLine {
            method: HttpMethod::Get,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };

        let (input, request_line) = request_line(&mut input).unwrap();
        assert_eq!(expected_request_line, request_line);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    #[test]
    fn test_request_line_parsing_get_method_without_line_ending() {
        let mut input: &[u8] = b"GET /data/test.html HTTP/1.1";

        let expected_request_line = RequestLine {
            method: HttpMethod::Get,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };

        let (input, request_line) = request_line(&mut input).unwrap();
        assert_eq!(expected_request_line, request_line);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    #[test]
    fn test_request_line_parsing_post_method_with_line_ending() {
        let mut input: &[u8] = b"POST /files/test.html HTTP/1.1\n";

        let expected_request_line = RequestLine {
            method: HttpMethod::Post,
            uri: UriPath::new("/files/test.html"),
            http_version: HttpVersion::V1_1,
        };

        let (input, request_line) = request_line(&mut input).unwrap();
        assert_eq!(expected_request_line, request_line);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    // ---- -- HTTP Headers -- ---- \\

    #[test]
    fn test_http_header_parsing() {
        let mut input: &[u8] = b"Host: www.test101.com\r\n";

        let expected_headers = (
            "Host".to_string(), "www.test101.com".to_string()
        );

        let (input, header) = header(&mut input).unwrap();
        assert_eq!(expected_headers, header);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    // ---- -- Parse HTTP Request metadata -- ---- \\

    #[test]
    fn test_request_metadata_parsing() {
        let expected_request_line = RequestLine {
            method: HttpMethod::Get,
            uri: UriPath::new("/data/test.html"),
            http_version: HttpVersion::V1_1,
        };
        let expected_headers = HashMap::from([
            ("Accept".to_string(), "image/gif, image/jpeg, *//*".to_string()),
            ("Host".to_string(), "www.test101.com".to_string()),
            ("Accept-Language".to_string(), "en-us".to_string()),
            ("Accept-Encoding".to_string(), "gzip, deflate".to_string()),
            ("User-Agent".to_string(), "Mozilla/4.0".to_string()),
            ("Content-Length".to_string(), "35".to_string()),
        ]);

        let request_lines = [ "GET /data/test.html HTTP/1.1",
            "Host: www.test101.com",
            "Accept: image/gif, image/jpeg, *//*",
            "Accept-Language: en-us",
            "Accept-Encoding: gzip, deflate",
            "User-Agent: Mozilla/4.0",
            "Content-Length: 35",
        ];

        let plain_headers: String = request_lines.join("\r\n");

        let (input, data) = parse_request_metadata(&mut plain_headers.as_bytes()).unwrap();
        let (request_line, headers) = data;

        assert_eq!(expected_request_line, request_line);
        assert_eq!(expected_headers, headers);


        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    // ---- -- Request -- ---- \\

    #[test]
    fn test_complex_request_with_body_parsing() {
        let expected_request = HttpRequest {
            request_line: RequestLine {
                method: HttpMethod::Get,
                uri: UriPath::new("/data/test.html"),
                http_version: HttpVersion::V1_1,
            },
            headers: HashMap::from([
                ("Host".to_string(), "www.test101.com".to_string()),
                ("Accept".to_string(), "image/gif, image/jpeg, *//*".to_string()),
                ("Accept-Language".to_string(), "en-us".to_string()),
                ("Accept-Encoding".to_string(), "gzip, deflate".to_string()),
                ("Content-Length".to_string(), "35".to_string()),
                ("User-Agent".to_string(), "Mozilla/4.0".to_string()),
            ]),
            body: Some("bookId=12345&author=Tan+Ah+Teck".as_bytes().to_vec()),
        };

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

        let (input, request) = request(&mut plain_request.as_bytes()).unwrap();
        assert_eq!(expected_request, request);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    #[test]
    fn test_complex_request_without_body_parsing() {
        let expected_request = HttpRequest {
            request_line: RequestLine {
                method: HttpMethod::Post,
                uri: UriPath::new("/data/test.html"),
                http_version: HttpVersion::V1_1,
            },
            headers: HashMap::from([
                ("Host".to_string(), "www.test101.com".to_string()),
                ("Accept-Language".to_string(), "en-us".to_string()),
                ("Accept-Encoding".to_string(), "gzip, deflate".to_string()),
                ("User-Agent".to_string(), "Mozilla/4.0".to_string()),
                ("Content-Length".to_string(), "0".to_string()),
            ]),
            body: None,
        };

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

        let (input, request) = request(&mut plain_request.as_bytes()).unwrap();
        assert_eq!(expected_request, request);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

    #[test]
    fn test_complex_request_http_get_empty_request() {
        let expected_request = HttpRequest {
            request_line: RequestLine {
                method: HttpMethod::Get,
                uri: UriPath::new(""),
                http_version: HttpVersion::V1_1,
            },
            headers: HashMap::new(),
            body: None,
        };

        let request_lines = ["GET / HTTP/1.1", "\r\n", ""];
        let plain_request: String = request_lines.join("\r\n");

        let (input, request) = request(&mut plain_request.as_bytes()).unwrap();
        assert_eq!(expected_request, request);

        let expected_input = b"";
        assert_eq!(input, expected_input);
    }

}
