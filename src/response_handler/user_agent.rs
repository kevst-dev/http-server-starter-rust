use std::collections::HashMap;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::response_handler::Handler;

pub struct UserAgentHandler;

impl Handler for UserAgentHandler {
    fn handle(request: &HttpRequest) -> HttpResponse {
        let status_code = "200";
        let body = request.headers.get("User-Agent").unwrap();
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.to_string()))
    }
}
