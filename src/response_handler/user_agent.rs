use std::collections::HashMap;

use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;

pub struct UserAgentHandler;

impl Handler<()> for UserAgentHandler {
    fn handle(request: &HttpRequest, _path: ()) -> HttpResponse {
        let header_expected = "User-Agent".to_lowercase();

        let status_code = "200";
        let body = request.headers.get(&header_expected).unwrap().trim();
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.as_bytes()))
    }
}
