use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use nom::AsBytes;

use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;
use crate::response_handler::PathNotFoundHandler;

pub struct GetFileHandler;

impl Handler<PathBuf> for GetFileHandler {
    fn handle(request: &HttpRequest, path_dir: PathBuf) -> HttpResponse {
        let file_name = request.uri().data();
        let path_file = path_dir.join(file_name);

        let http_response: HttpResponse = match fs::read(path_file) {
            Ok(file) => {
                let status_code = "200";
                let body = file.as_bytes();
                let mut headers = HashMap::new();
                headers.insert(
                    "Content-type".to_string(),
                    "application/octet-stream".to_string(),
                );

                HttpResponse::new(status_code, Some(headers), Some(body))
            }
            Err(_) => PathNotFoundHandler::handle(request, ()),
        };

        http_response
    }
}
