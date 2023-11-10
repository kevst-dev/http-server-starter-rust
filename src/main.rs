use std::io::{BufRead, BufReader};
use std::str::FromStr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;

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

const BUFFER_SIZE: usize = 255;

async fn handle_client(mut stream: TcpStream) -> Result<(), String>{
    let mut buffer = vec![0; BUFFER_SIZE];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) =>{
            let request = String::from_utf8(buffer.into_iter().take(bytes_read).collect())
                .map_err(|e| format!("Error decoding UTF-8: {}", e))?;

            let request = HttpRequest::from(request.to_string());

            println!("request: {:?}", request);

            Router::route(request, &mut stream).await;

        },
        Err(e) => {
            return Err(format!("Failed to read data:{}", e));
        }
    };

    Ok(())

    /*
    let mut reader = BufReader::new(&stream);

    let bytes_request = reader.fill_buf().unwrap();
    let request = String::from_utf8_lossy(bytes_request);
    let request = HttpRequest::from(request.to_string());

    println!("request: {:?}", request);

    Router::route(request, &mut stream);
    */
}

#[tokio::main]
async fn main() {
    println!("Server is starting...");

    let url = "127.0.0.1:4221/";
    let url = ParseUrl::from_str(url).unwrap();

    let listener = match TcpListener::bind(url.get_host()).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to the port: {}", e);
            return;
        }
    };

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);

                continue;
            }
        };

        println!("Accepting connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                println!("Connection with {} failed: {}", addr, e);

            }

        });
    }
}
