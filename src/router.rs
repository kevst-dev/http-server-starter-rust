use crate::http_request::{HttpMethod, HttpRequest};
use crate::http_response::HttpResponse;
use crate::response_handler;
use crate::response_handler::Handler;


use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(request: HttpRequest, stream: &mut impl Write) {
        match request.method {
            // If GET request
            HttpMethod::Get => {
                Self.route_get(request, stream);
            }
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request);
                let _ = response.send_response(stream);
            }
        }
    }

    fn route_get(&self, request: HttpRequest, stream: &mut impl Write) {
        match request.resource.path().as_str() {
            "/" => {
                let response: HttpResponse =
                    response_handler::PathDefaultHandler::handle(&request);

                let _ = response.send_response(stream);
            },
            "/echo" => {
                let response: HttpResponse =
                    response_handler::EchoHandler::handle(&request);

                let _ = response.send_response(stream);
            },

            "/user-agent" => {
                let response: HttpResponse =
                    response_handler::UserAgentHandler::handle(&request);

                let _ = response.send_response(stream);
            },
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request);
                let _ = response.send_response(stream);
            },
        }
    }
}
