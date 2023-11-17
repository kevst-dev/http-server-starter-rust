use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::response_handler::Handler;

pub struct PostFileHandler;

impl Handler<PathBuf> for PostFileHandler {
    fn handle(request: &HttpRequest, path_dir: PathBuf) -> HttpResponse {
        let body = String::from_utf8(request.clone().body.unwrap());
        let file_name = request.resource.data();
        let path_file = path_dir.join(file_name);

        // crear archivo y escribir contenido
        fs::write(path_file, body.unwrap()).unwrap();

        let status_code = "201";
        let body = "file saved".as_bytes();
        let mut headers = HashMap::new();
        headers.insert(
            "Content-type".to_string(),
            "application/octet-stream".to_string(),
        );

        HttpResponse::new(status_code, Some(headers), Some(body))
    }
}
