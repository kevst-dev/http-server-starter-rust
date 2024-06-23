use std::fs;
use std::io::Write;

use reqwest::Client;

#[tokio::test]
async fn test_http_server_status_code_200() {
    let http_client = Client::new();
    let response = http_client
        .get("http://localhost:4221/")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    assert_eq!(response.headers()["content-type"], "text/plain");
    assert_eq!(response.headers()["content-length"], "37");

    let body = response.text().await.unwrap();
    assert_eq!(body, "Todo en orden pero no conozco la ruta");
}

#[tokio::test]
async fn test_http_server_command_echo() {
    let data_array = [
        "",
        "linux",
        "Coo/dooby",
        "monkey/Coo-donkey",
        "monkey/monkey-237",
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

        let body = response.text().await.unwrap();
        assert_eq!(body, *data);
    }
}

#[tokio::test]
async fn test_http_server_command_user_agent() {
    let users_agents = ["curl/7.68.0", "Mozilla/5.0", "Linux", "Monkey/Horsey"];

    for user_agent in &users_agents {
        let http_client =
            Client::builder().user_agent(*user_agent).build().unwrap();

        let response = http_client
            .get("http://localhost:4221/user-agent")
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_eq!(
            response.headers()["content-length"],
            user_agent.len().to_string()
        );

        let body = response.text().await.unwrap();
        assert_eq!(body, *user_agent);
    }
}

// Ejecuta el servidor como 'just run -- --directory tests/data'
#[tokio::test]
async fn test_http_server_command_get_files() {
    let name_files = ["index.html", "IMG_5491.JPG"];
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
