use reqwest::Client;
use flate2::read::GzDecoder;

use std::io::Read;

/*
* Comprueba que el servidor responde al comando 'echo'
* con una respuesta correcta.
*
* Caso 1 (Sin el header 'Accept-Encoding'):
* [test] Responde con el codiigo de estatos 200
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
* [test] Responde con el body correcto, que es el valor de 'data'
*
* Example:
* $ curl -v http://localhost:4221/echo/strawberry
*/

#[tokio::test]
async fn test_http_get_command_echo_without_accept_encoding() {
    let data_array = [
        "",
        "linux",
        "Coo/dooby",
        "monkey/Coo-donkey",
        "monkey/monkey-237",
        "strawberry",
    ];
    let host = String::from("http://localhost:4221");

    // Para cada dato en el array, realizar una solicitud HTTP
    for data in &data_array {
        let path = format!("echo/{}", data);
        let url = format!("{}/{}", host, path);

        let http_client = Client::new();
        let response = http_client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);

        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(
            response.headers()["content-length"],
            data.len().to_string()
        );
        assert!(!response.headers().contains_key("content-encoding"));

        let body = response.text().await.unwrap();
        assert_eq!(body, *data);
    }
}

/*
* Comprueba que el servidor responde al comando 'echo'
* con una respuesta correcta.
*
* Caso 1 (con el header 'Accept-Encoding' valido):
* [test] Responde con el codiigo de estatos 200
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
* [test] Responde con el body correcto, que es el valor de 'data'
*
* Example:
* $ curl -v http://localhost:4221/echo/strawberry -H "Accept-Encoding: gzip"
*/

#[tokio::test]
async fn test_http_get_command_echo_with_accept_encoding_valid() {
    let data_array = [
        "linux",
        "abc"
    ];
    let host = String::from("http://localhost:4221");

    // Para cada dato en el array, realizar una solicitud HTTP
    for data in &data_array {
        let path = format!("echo/{}", data);
        let url = format!("{}/{}", host, path);

        let http_client = Client::new();

        let response = http_client
            .get(&url)
            .header("Accept-Encoding", "none, gzip, invalid")
            .send().await.unwrap();

        assert_eq!(response.status(), 200);

        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(response.headers()["content-encoding"], "gzip");

        // decodificando el body, que viene en gzip
        let body_bytes = response.bytes().await.unwrap();
        let mut decoder = GzDecoder::new(&body_bytes[..]);
        let mut body = String::new();
        decoder.read_to_string(&mut body).unwrap();

        assert_eq!(body, *data);
    }
}

/*
* Comprueba que el servidor responde al comando 'echo'
* con una respuesta correcta.
*
* Caso 1 (con el header 'Accept-Encoding' invalido):
* [test] Responde con el codiigo de estatos 200
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
* [test] Responde con el body correcto, que es el valor de 'data'
*
* Example:
* $ curl -v http://localhost:4221/echo/strawberry -H "Accept-Encoding: invalid"
*/

#[tokio::test]
async fn test_http_get_command_echo_with_accept_encoding_invalid() {
    let data_array = [
        "linux",
        "abc"
    ];
    let host = String::from("http://localhost:4221");

    // Para cada dato en el array, realizar una solicitud HTTP
    for data in &data_array {
        let path = format!("echo/{}", data);
        let url = format!("{}/{}", host, path);

        let http_client = Client::new();

        let response = http_client
            .get(&url)
            .header("Accept-Encoding", "invalid, none")
            .send().await.unwrap();

        assert_eq!(response.status(), 200);

        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(
            response.headers()["content-length"],
            data.len().to_string()
        );
        assert!(!response.headers().contains_key("content-encoding"));

        let body = response.text().await.unwrap();
        assert_eq!(body, *data);
    }
}
