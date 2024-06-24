use reqwest::Client;

/*
* Comprueba que el servidor responde estatus 200
* con solicitudes sin path.
*
* Example:
* $ curl -v http://localhost:4221/
*/

#[tokio::test]
async fn test_http_get_path_not_exist() {
    let http_client = Client::new();
    let response = http_client
        .get("http://localhost:4221/")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    // headers
    assert_eq!(response.headers()["content-type"], "text/plain");
    assert_eq!(response.headers()["content-length"], "37");

    let body = response.text().await.unwrap();
    assert_eq!(body, "Todo en orden pero no conozco la ruta");
}
