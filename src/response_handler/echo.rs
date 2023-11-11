use std::collections::HashMap;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::response_handler::Handler;

pub struct EchoHandler;

impl Handler<()> for EchoHandler {
    fn handle(request: &HttpRequest, _path: ()) -> HttpResponse {
        let status_code = "200";
        let body = request.resource.data();
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.as_bytes()))
    }
}
