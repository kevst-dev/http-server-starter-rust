use reqwest::Client;

/*
* Comprueba que el servidor responde estatus 404
* con solicitudes con paths que no existen.
*
* Example:
* $ curl -v http://localhost:4221/banana
*/

#[tokio::test]
async fn test_http_get_path_not_found() {
    let data_array = ["banana", "error", "not-found", "Horsey/Horsey-scooby"];
    let host = String::from("http://localhost:4221");

    // Para cada dato en el array, realizar una solicitud HTTP
    for data in &data_array {
        let url = format!("{}/{}", host, data);

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
