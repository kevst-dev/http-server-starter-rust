use reqwest::Client;

/*
* Comprueba que el servidor responde al comando 'user-agent'
* con el valor de la cabecera 'User-Agent' como body.
*
* [test] Responde con el codigo de estatos 200
* [test] Responde con el header {'Content-Type': 'text/plain'}
* [test] Responde con el header {'Content-Length': '<length>'}
* [test] Responde con el valor de la cabecera 'User-Agent' como body
*
* Example:
* $ curl -v http://localhost:4221/user-agent -H "User-Agent: grape/apple-blueberry"
*/

#[tokio::test]
async fn test_http_server_command_user_agent() {
    let users_agents = ["curl/7.68.0", "Mozilla/5.0", "Linux", "Monkey/Horsey"];

    for user_agent in &users_agents {
        // agregando el header user-agent
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
