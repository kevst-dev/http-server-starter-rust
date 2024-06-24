use reqwest::Client;

/*
* Comprueba que el servidor responde al comando 'echo'
* con una respuesta correcta.
*
* [test] Responde con el codiigo de estatos 200
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
* [test] Responde con el body correcto, que es el valor de 'data'
*
* Example:
* $ curl -v http://localhost:4221/echo/strawberry
*/

#[tokio::test]

async fn test_http_get_command_echo() {
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

        let body = response.text().await.unwrap();
        assert_eq!(body, *data);
    }
}
