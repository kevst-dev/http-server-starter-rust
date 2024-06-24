use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;

use flate2::Compression;
use flate2::write::GzEncoder;

use std::io::Write;

pub struct EchoHandler;

impl Handler<()> for EchoHandler {
    fn handle(request: &HttpRequest, _path: ()) -> HttpResponse {
        let mut headers = request.get_headers_for_the_response();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        let body: Vec<u8> = if headers.contains_key("Content-Encoding") {
            let encoding = headers.get("Content-Encoding").unwrap();
            let mut body_bytes = Vec::new();

            if encoding == "gzip" {
                let body = request.uri().data().as_bytes().to_vec();

                let mut encoder = GzEncoder::new(vec![], Compression::default());
                encoder.write_all(&body).unwrap();

                let compressed_buf = encoder.finish().unwrap();

                body_bytes.extend_from_slice(compressed_buf.as_slice());
            }

            body_bytes
        } else {
            request.uri().data().as_bytes().to_vec()
        };

        let status_code = "200";


        HttpResponse::new(status_code, Some(headers), Some(&body))
    }
}
