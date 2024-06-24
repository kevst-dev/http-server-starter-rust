use std::fs;
use std::io::Write;

use reqwest::Client;

/*
* Comprueba que el servidor responde al comando 'files'
* correctamente.
*
* Caso 1 (Archivo existente):
*
* $ curl -v http://localhost:4221/files/raspberry_blueberry_mango_raspberry
*
* [test] Responde con el codigo de estatos 200
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
* [test] Responde con el valor binario del archivo como body
*/

// Ejecuta el servidor como 'just run -- --directory tests/data'

#[tokio::test]
async fn test_http_get_command_files_exist_file() {
    let name_files = ["index.html", "Rick_and_Morty.jpg"];
    let host = String::from("http://localhost:4221");

    // Crea un directorio temporal
    let temp_dir = std::env::temp_dir();

    for name_file in &name_files {
        let path = format!("files/{}", name_file);
        let url = format!("{}/{}", host, path);

        let http_client = Client::new();
        let response = http_client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 200);

        assert_eq!(
            response.headers()["content-type"],
            "application/octet-stream"
        );

        let content_length = response.headers()["content-length"].clone();
        let temp_file_path = temp_dir.join(name_file);

        // Guarda el cuerpo de la respuesta HTTP en un archivo local
        let body_bytes = response.bytes().await.unwrap();
        let mut file = fs::File::create(&temp_file_path).unwrap();
        file.write_all(&body_bytes).unwrap();

        assert_eq!(content_length, file.metadata().unwrap().len().to_string());

        fs::remove_file(temp_file_path).unwrap();
    }
}

/*
* Comprueba que el servidor responde al comando 'files'
* correctamente.
*
* Caso 2 (Archivo no existente):
*
* $ curl -v http://localhost:4221/files/non-existentpineapple_banana_pineapple_apple
*
* [test] Responde con el codigo de estatos 404
*/

#[tokio::test]
async fn test_http_get_command_files_no_exist_file() {
    let name_files = ["banana", "error", "not-exist"];
    let host = String::from("http://localhost:4221");

    // Para cada dato en el array, realizar una solicitud HTTP
    for name_file in &name_files {
        let path = format!("files/{}", name_file);
        let url = format!("{}/{}", host, path);

        let http_client = Client::new();
        let response = http_client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 404);

        // headers
        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(response.headers()["content-length"], "39");

        let body = response.text().await.unwrap();
        assert_eq!(body, "No existe el recurso que ha sido pedido");
    }
}
