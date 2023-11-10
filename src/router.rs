use crate::http_request::{HttpMethod, HttpRequest};
use crate::http_response::HttpResponse;
use crate::response_handler;
use crate::response_handler::Handler;

use tokio::net::TcpStream;

use std::io::prelude::*;

pub struct Router;

impl Router {
    pub async fn route(request: HttpRequest, stream: &mut TcpStream) {
        match request.method {
            // If GET request
            HttpMethod::Get => {
                Self.route_get(request, stream).await;
            }
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request);
                let _ = response.send_response(stream).await.unwrap();
            }
        }
    }

    async fn route_get(&self, request: HttpRequest, stream: &mut TcpStream) {
        match request.resource.path().as_str() {
            "/" => {
                let response: HttpResponse =
                    response_handler::PathDefaultHandler::handle(&request);

                let _ = response.send_response(stream).await.unwrap();
            },
            "/echo" => {
                let response: HttpResponse =
                    response_handler::EchoHandler::handle(&request);

                let _ = response.send_response(stream).await.unwrap();
            },

            "/user-agent" => {
                let response: HttpResponse =
                    response_handler::UserAgentHandler::handle(&request);

                let _ = response.send_response(stream).await.unwrap();
            },
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request);

                let _ = response.send_response(stream).await.unwrap();
            },
        }
    }
}
