use std::collections::HashMap;

use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;

pub struct PathNotFoundHandler;

impl Handler<()> for PathNotFoundHandler {
    fn handle(_request: &HttpRequest, _path: ()) -> HttpResponse {
        let status_code = "404";
        let body = "No existe el recurso que ha sido pedido";
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.as_bytes()))
    }
}
