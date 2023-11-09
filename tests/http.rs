use std::process::Command;

use http_server_starter_rust::http_request::{HttpMethod, HttpRequest, Resource};
use http_server_starter_rust::http_response::{HttpResponse};

fn define_curl_cli(args: Vec<&str>) -> Command {
    let mut curl_command = Command::new("curl");

    let mut combined_args = vec!["-v"];
    combined_args.extend(args.iter().cloned());

    curl_command.args(combined_args);
    curl_command
}

fn format_stdout(output: String) -> (String, String) {
    // Limpia y formatea la información innecesaria
    // de la salida del comando curl.

    fn clean_lines(lines: &str, prefix: char) -> String {
        lines
            .lines()
            .filter(|line| line.starts_with(prefix))
            .map(|line| {
                let mut cleaned_line = line.to_string();
                cleaned_line.remove(0); // Remove the prefix character
                cleaned_line.remove(0); // Remove the space
                cleaned_line
            })
            .collect::<Vec<String>>()
            .join("\r\n")
    }

    let request = clean_lines(&output, '>');
    let response = clean_lines(&output, '<');

    (format!("{}\r\n", request), response)
}

#[test]
fn test_http_status_code_200_none_path() {
    let args = vec!["http://localhost:4221/"];
    let mut curl_cli = define_curl_cli(args);

    let output = curl_cli.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    let (request, response) = format_stdout(stderr.to_string());
    let request = HttpRequest::from(request.clone());

    assert_eq!(HttpMethod::Get, request.method);
    assert!(response.contains("HTTP/1.1 200 OK"));
}

#[test]
fn test_http_status_code_200_echo_path() {
    let args = vec!["http://localhost:4221/echo/linux"];
    let mut curl_cli = define_curl_cli(args);

    let output = curl_cli.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    let (request, response) = format_stdout(stderr.to_string());
    let request = HttpRequest::from(request.clone());

    assert_eq!(HttpMethod::Get, request.method);
    assert_eq!(request.resource, Resource::Path("/echo/linux".to_string()));
    assert!(response.contains("HTTP/1.1 200 OK"));
}

#[test]
fn test_http_status_code_404() {
    let args = vec!["-X", "POST","http://localhost:4221/data.xml"];
    let mut curl_cli = define_curl_cli(args);

    let output = curl_cli.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    let (request, response) = format_stdout(stderr.to_string());
    let request = HttpRequest::from(request.clone());

    assert_eq!(HttpMethod::Post, request.method);
    assert!(response.contains("HTTP/1.1 404 Not Found"));
}
