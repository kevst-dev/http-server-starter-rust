use std::collections::HashMap;

use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;

pub struct EchoHandler;

impl Handler<()> for EchoHandler {
    fn handle(request: &HttpRequest, _path: ()) -> HttpResponse {
        let status_code = "200";
        let body = request.uri().data();
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.as_bytes()))
    }
}
