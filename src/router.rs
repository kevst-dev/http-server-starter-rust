use crate::http_request::{HttpMethod, HttpRequest};
use crate::http_response::HttpResponse;
use crate::response_handler;
use crate::response_handler::Handler;

use std::path::PathBuf;
use tokio::net::TcpStream;

pub struct Router;

impl Router {
    pub async fn route(
        request: HttpRequest,
        stream: &mut TcpStream,
        path_dir: PathBuf,
    ) {
        match request.method {
            HttpMethod::Get => {
                Self.route_get(request, stream, path_dir).await;
            }
            HttpMethod::Post => {
                Self.route_post(request, stream, path_dir).await;
            }
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request, ());
                response.send_response(stream).await.unwrap();
            }
        }
    }

    async fn route_get(
        &self,
        request: HttpRequest,
        stream: &mut TcpStream,
        path_dir: PathBuf,
    ) {
        match request.resource.path().as_str() {
            "/" => {
                let response: HttpResponse =
                    response_handler::PathDefaultHandler::handle(&request, ());

                response.send_response(stream).await.unwrap();
            }
            "/echo" => {
                let response: HttpResponse =
                    response_handler::EchoHandler::handle(&request, ());

                response.send_response(stream).await.unwrap();
            }
            "/user-agent" => {
                let response: HttpResponse =
                    response_handler::UserAgentHandler::handle(&request, ());

                response.send_response(stream).await.unwrap();
            }
            "/files" => {
                let response: HttpResponse =
                    response_handler::GetFileHandler::handle(
                        &request, path_dir,
                    );

                response.send_response(stream).await.unwrap();
            }
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request, ());

                response.send_response(stream).await.unwrap();
            }
        }
    }

    async fn route_post(
        &self,
        request: HttpRequest,
        stream: &mut TcpStream,
        path_dir: PathBuf,
    ) {
        match request.resource.path().as_str() {
            "/files" => {
                let response: HttpResponse =
                    response_handler::PostFileHandler::handle(
                        &request, path_dir,
                    );

                response.send_response(stream).await.unwrap();
            }
            _ => {
                let response: HttpResponse =
                    response_handler::PathNotFoundHandler::handle(&request, ());

                response.send_response(stream).await.unwrap();
            }
        }
    }
}
