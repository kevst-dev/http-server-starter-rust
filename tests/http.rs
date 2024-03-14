use std::fs;
use std::path::Path;

use reqwest::Client;

#[tokio::test]
async fn test_http_status_code_404() {
    let data_array = ["data.xml", "error", "not-found", "Horsey/Horsey-scooby"];
    let host = String::from("http://localhost:4221");

    // Para cada dato en el array, realizar una solicitud HTTP
    for data in &data_array {
        let url = format!("{}/{}", host, data);

        let http_client = Client::new();
        let response = http_client.get(&url).send().await.unwrap();

        assert_eq!(response.status(), 404);

        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(response.headers()["content-length"], "39");

        let body = response.text().await.unwrap();
        assert_eq!(body, "No existe el recurso que ha sido pedido");
    }
}

// Ejecuta el servidor como 'just run -- --directory tests/data'
#[tokio::test]
async fn test_http_server_command_post_files() {
    let this_file = file!();
    let this_file = std::path::Path::new(this_file);
    let folder = this_file.parent().unwrap().join("data");

    let file_name = "index.html";
    let file_path = folder.join(file_name);
    let file_name_result = "index_test.html";
    let host = String::from("http://localhost:4221");
    let url = format!("{}/files/{}", host, file_name_result);

    // Lee el contenido del archivo y conviértelo en bytes
    let file_content = fs::read(file_path).unwrap();

    let http_client = Client::new();
    let response = http_client
        .post(&url)
        .body(file_content)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    assert_eq!(
        response.headers()["content-type"],
        "application/octet-stream"
    );
    assert_eq!(
        response.headers()["content-length"],
        file_name.len().to_string()
    );

    let result_path = folder.join(file_name_result);
    assert!(Path::new(&result_path).exists());

    fs::remove_file(&result_path).unwrap();
}
