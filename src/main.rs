use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

mod errors;
mod http_request;
mod http_response;
mod parse_url;
mod response_handler;
mod router;
mod url_path;

use http_request::HttpRequest;

use parse_url::ParseUrl;
use router::Router;

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let bytes_request = reader.fill_buf().unwrap();
    let request = String::from_utf8_lossy(bytes_request);
    let request = HttpRequest::from(request.to_string());

    println!("request: {:?}", request);

    Router::route(request, &mut stream);
}

fn main() {
    let url = "127.0.0.1:4221/";
    let url = ParseUrl::from_str(url).unwrap();

    let listener = TcpListener::bind(url.get_host()).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
