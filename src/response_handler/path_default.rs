use std::collections::HashMap;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::response_handler::Handler;

pub struct PathDefaultHandler;

impl Handler<()> for PathDefaultHandler {
    fn handle(_request: &HttpRequest, _path: ()) -> HttpResponse {
        let status_code = "200";
        let body = "Todo en orden pero no conozco la ruta";
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.as_bytes()))
    }
}
