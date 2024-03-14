use std::collections::HashMap;

use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::response_handler::Handler;

pub struct UserAgentHandler;

impl Handler<()> for UserAgentHandler {
    fn handle(request: &HttpRequest, _path: ()) -> HttpResponse {
        let user_agent = request
            .headers
            .get("User-Agent")
            .unwrap_or_else(|| request.headers.get("user-agent").unwrap());

        let status_code = "200";
        let body = user_agent.trim();
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        HttpResponse::new(status_code, Some(headers), Some(body.as_bytes()))
    }
}
