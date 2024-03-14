use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

mod errors;
mod http;
mod parse_url;
mod response_handler;
mod router;

use http::HttpRequest;
use parse_url::ParseUrl;
use router::Router;

const BUFFER_SIZE: usize = 1024 * 8;

async fn handle_client(
    mut stream: TcpStream,
    path_dir: PathBuf,
) -> Result<(), String> {
    let mut buffer = vec![0; BUFFER_SIZE];

    match stream.read(&mut buffer).await {
        Ok(bytes_read) => {
            let request = HttpRequest::from(&buffer[0..bytes_read]);

            Router::route(request, &mut stream, path_dir).await;
        }
        Err(e) => {
            return Err(format!("Failed to read data:{}", e));
        }
    };

    Ok(())
}

fn parse_args(args: Vec<String>) -> PathBuf {
    if args.len() < 2 {
        return PathBuf::from(".");
    }

    let _bin_dir = &args[0];
    let arg_flag = &args[1];
    let arg_dir = &args[2];

    if arg_flag != "--directory" {
        panic!("Expected --directory argument");
    }

    // convertir file en Path
    let arg_dir =
        PathBuf::from_str(arg_dir).expect("Failed to parse file path");

    if !arg_dir.is_dir() {
        panic!("Expected directory path");
    }

    arg_dir
}

#[tokio::main]
async fn main() {
    // Read the --directory <directory> argument
    let args: Vec<String> = std::env::args().collect();
    let directory = parse_args(args);
    let directory = Arc::new(directory);

    println!("Directory: {:?}", &directory);

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
        let directory = Arc::clone(&directory);

        let (stream, addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);

                continue;
            }
        };

        println!("Accepting connection from {}", addr);

        tokio::spawn(async move {
            let directory = Arc::clone(&directory);

            if let Err(e) = handle_client(stream, directory.to_path_buf()).await
            {
                println!("Connection with {} failed: {}", addr, e);
            }
        });
    }
}
