use std::collections::HashMap;
use std::io::Result;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: String,
    status_text: String,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            headers: None,
            body: None,
        }
    }
}

impl HttpResponse {
    pub fn new(
        status_code: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<&[u8]>,
    ) -> HttpResponse {
        let mut response: HttpResponse = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        };

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut headers = HashMap::new();
                headers.insert(
                    "Content-type".to_string(),
                    "text/html".to_string(),
                );

                Some(headers)
            }
        };

        response.status_text = match response.status_code.as_str() {
            "200" => "OK".to_string(),
            "400" => "Bad Request".to_string(),
            "404" => "Not Found".to_string(),
            "500" => "Internal Server Error".to_string(),
            _ => "Not Found".to_string(),
        };

        response.body = body.map(|b| b.to_vec());

        response
    }

    pub async fn send_response(
        &self,
        write_stream: &mut TcpStream,
    ) -> Result<()> {
        let response = self.clone();
        let response_bytes = Vec::<u8>::from(response); 
        // let response_string: String = String::from(response);

        write_stream
            .write_all(response_bytes.as_slice())
            .await
            .unwrap();

        Ok(())
    }

    #[allow(dead_code)]
    fn version(&self) -> String {
        self.version.to_string()
    }

    #[allow(dead_code)]
    fn status_code(&self) -> String {
        self.status_code.to_string()
    }

    #[allow(dead_code)]
    fn status_text(&self) -> String {
        self.status_text.to_string()
    }

    fn headers(&self) -> String {
        match &self.headers {
            Some(map) => {
                let header_string: String = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}\r\n", k, v))
                    .collect();

                header_string
            }
            None => String::new(),
        }
    }

    pub fn body(&self) -> Vec<u8> {
        match &self.body {
            Some(b) => b.clone(),
            None => Vec::new(),
        }
    }
}

impl From<HttpResponse> for Vec<u8> {
    fn from(response: HttpResponse) -> Vec<u8> {
        /*
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            response.version,
            response.status_code,
            response.status_text,
            response.headers(),
            response.body().len(),
            response.body(),
        )
        */
        let mut result = Vec::new();

        result.extend_from_slice(response.version.as_bytes());
        result.extend_from_slice(b" ");
        result.extend_from_slice(response.status_code.as_bytes());
        result.extend_from_slice(b" ");
        result.extend_from_slice(response.status_text.as_bytes());
        result.extend_from_slice(b"\r\n");

        if let Some(headers) = &response.headers {
            for (key, value) in headers {
                result.extend_from_slice(key.as_bytes());
                result.extend_from_slice(b": ");
                result.extend_from_slice(value.as_bytes());
                result.extend_from_slice(b"\r\n");
            }
        }
        result.extend_from_slice(b"Content-Length: ");
        result.extend_from_slice(response.body().len().to_string().as_bytes());

        result.extend_from_slice(b"\r\n\r\n");
        
        if let Some(body) = &response.body {
            result.extend_from_slice(body);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_response() {
        let default_response = HttpResponse::default();

        assert_eq!(default_response.version, "HTTP/1.1");
        assert_eq!(default_response.status_code, "200");
        assert_eq!(default_response.status_text, "OK");
        assert_eq!(default_response.headers, None);
        assert_eq!(default_response.body, None);
    }

    #[test]
    fn test_response_creation_200() {
        let status_code = "200";
        let body = "Item was shipped on 21st Dec 2020";
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/plain".to_string());

        let expected_response = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: status_code.to_string(),
            status_text: "OK".to_string(),
            headers: Some(headers.clone()),
            body: Some(body.as_bytes().to_vec()),
        };

        let response = HttpResponse::new(
            status_code,
            Some(headers.clone()),
            Some(body.as_bytes()),
        );

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let status_code = "404";
        let body = "Item was shipped on 21st Dec 2020";
        let mut headers = HashMap::new();
        headers.insert("Content-type".to_string(), "text/html".to_string());

        let expected_response = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: status_code.to_string(),
            status_text: "Not Found".to_string(),
            headers: Some(headers.clone()),
            body: Some(body.as_bytes().to_vec()),
        };

        let response =
            HttpResponse::new(status_code, None, Some(body.as_bytes()));
        assert_eq!(response, expected_response);
    }
}
