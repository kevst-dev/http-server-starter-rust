use crate::http_request::{HttpMethod, HttpRequest};
use crate::http_response::HttpResponse;

use std::io::prelude::*;
use std::collections::HashMap;

pub struct Router;

impl Router {
    pub fn route(request: HttpRequest, stream: &mut impl Write) {
        match request.method {
            // If GET request
            HttpMethod::Get => match &request.resource {
                crate::http_request::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();

                    match route[1] {
                        "echo" => {
                            let status_code = "200";
                            let body = s.clone().replace(&format!("/{}/", route[1]), "");
                            let mut headers = HashMap::new();
                            headers.insert(
                                "Content-type".to_string(), "text/plain".to_string()
                            );

                            let response = HttpResponse::new(
                                status_code,
                                Some(headers),
                                Some(body.to_string()),
                            );

                            let _ = response.send_response(stream);
                        }
                        // if '/'
                        "" => {
                            let status_code = "200";
                            let body = "Todo en orden pero no conozco la ruta";
                            let mut headers = HashMap::new();
                            headers.insert(
                                "Content-type".to_string(), "text/plain".to_string()
                            );

                            let response = HttpResponse::new(
                                status_code,
                                Some(headers),
                                Some(body.to_string()),
                            );

                            let _ = response.send_response(stream);
                        },
                        _ => {
                            let status_code = "404";
                            let body = format!(
                                "HTTP {:?} method not supported", request.method
                            );
                            let mut headers = HashMap::new();
                            headers.insert(
                                "Content-type".to_string(), "text/plain".to_string()
                            );

                            let response = HttpResponse::new(
                                status_code,
                                Some(headers),
                                Some(body.to_string()),
                            );

                            let _ = response.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let status_code = "404";
                let body = format!(
                    "HTTP {:?} method not supported", request.method
                );
                let mut headers = HashMap::new();
                headers.insert(
                    "Content-type".to_string(), "text/plain".to_string()
                );

                let response = HttpResponse::new(
                    status_code,
                    Some(headers),
                    Some(body.to_string()),
                );

                let _ = response.send_response(stream);
            }
        }
    }
}
