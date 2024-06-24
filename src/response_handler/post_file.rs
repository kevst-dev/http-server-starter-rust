use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;

pub struct PostFileHandler;

impl Handler<PathBuf> for PostFileHandler {
    fn handle(request: &HttpRequest, path_dir: PathBuf) -> HttpResponse {
        let body = String::from_utf8(request.clone().body.unwrap());
        let file_name = request.uri().data();
        let path_file = path_dir.join(file_name);

        // crear archivo y escribir contenido
        fs::write(path_file, body.unwrap()).unwrap();

        let status_code = "201";
        // let body = "C".as_bytes();
        let mut headers = HashMap::new();
        headers.insert(
            "Content-type".to_string(),
            "application/octet-stream".to_string(),
        );

        HttpResponse::new(status_code, Some(headers), None)
    }
}
