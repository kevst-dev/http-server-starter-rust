use std::fs;
use std::io::Write;
use std::path::Path;

use reqwest::Client;

/*
* Comprueba que el servidor responde al comando 'files'
* correctamente.
*
* Caso 1 (Archivo existente):
*
* $ curl -v http://localhost:4221/files/raspberry_blueberry_mango_raspberry
* $ curl -v -X POST http://localhost:4221/files/mango_raspberry_raspberry_strawberry
*   -H "Content-Length: 55"
*   -H "Content-Type: application/octet-stream"
*   -d 'pear raspberry apple pear mango orange banana raspberry'
*
* [test] Responde con el codigo de estatos 201
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
*/

// Ejecuta el servidor como 'just run -- --directory tests/data'

#[tokio::test]
async fn test_http_post_command_files_exist_file() {
    let html_content = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Rick and Morty</title>
    </head>
    <body>
        <h1>Rick and Morty</h1>
    </body>
    </html>
    "#;

    // Crea un directorio temporal
    let temp_dir = std::env::temp_dir();

    // crea un archivo temporal
    let input_name_file = "index_input.html";
    let path_input_file = temp_dir.join(input_name_file);
    let mut file_input = fs::File::create(path_input_file.clone()).unwrap();
    file_input.write_all(html_content.as_bytes()).unwrap();

    // lee el contenido del archivo y convertirlo en bytes
    let input_bytes = fs::read(path_input_file.clone()).unwrap();

    let output_name_file = "index_output.html";
    let host = String::from("http://localhost:4221");
    let url = format!("{}/files/{}", host, output_name_file);

    let http_client = Client::new();
    let response = http_client
        .post(&url)
        .body(input_bytes.clone())
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
    assert_eq!(
        response.headers()["content-type"],
        "application/octet-stream"
    );

    let this_file = file!();
    let this_file = std::path::Path::new(this_file);
    let server_folder = this_file.ancestors().nth(2).unwrap().join("data");

    let result_path = server_folder.join(output_name_file);
    assert!(Path::new(&result_path).exists());

    fs::remove_file(path_input_file).unwrap();
    fs::remove_file(result_path).unwrap();
}
