use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

mod errors;

pub mod http_request;
use http_request::{HttpRequest, Resource};

mod parse_url;
use parse_url::ParseUrl;

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let bytes_request = reader.fill_buf().unwrap();
    let request = String::from_utf8_lossy(bytes_request);
    let request = HttpRequest::from(request.to_string());

    println!("request: {:?}", request);

    let response = match &request.resource {
        Resource::Path(path) => match path.as_str() {
            "/index.html" => "HTTP/1.1 200 OK\r\n\r\n",
            _ => "HTTP/1.1 404 Not Found\r\n\r\n",
        },
    };

    stream.write_all(response.as_bytes()).unwrap();
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
