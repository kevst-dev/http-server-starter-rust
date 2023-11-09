use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: String,
    status_text: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
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
        body: Option<String>,
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
                    "text/html".to_string()
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

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let response = self.clone();
        let response_string: String = String::from(response);

        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    fn version(&self) -> String { self.version.to_string() }
    fn status_code(&self) -> String { self.status_code.to_string() }
    fn status_text(&self) -> String { self.status_text.to_string() }

    fn headers(&self) -> String {
        match &self.headers {
            Some(map) => {
                let header_string: String = map
                    .iter()
                    .map(|(k, v)| format!("{}:{}\r\n", k, v))
                    .collect();

                header_string
            }
            None => String::new(),
        }
    }

    pub fn body(&self) -> String {
        match &self.body {
            Some(b) => b.to_string(),
            None => "".to_string(),
        }
    }
}

impl From<HttpResponse> for String {
   fn from(response: HttpResponse) -> String {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            response.version,
            response.status_code,
            response.status_text,
            response.headers(),
            response.body().len(),
            response.body(),
        )
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
        headers.insert(
            "Content-type".to_string(), "text/plain".to_string()
        );

        let expected_response = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: status_code.to_string(),
            status_text: "OK".to_string(),
            headers: Some(headers.clone()),
            body: Some(body.to_string()),
        };

        let response = HttpResponse::new(
            status_code,
            Some(headers.clone()),
            Some(body.to_string()),
        );

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let status_code = "404";
        let body = "Item was shipped on 21st Dec 2020";
        let mut headers = HashMap::new();
        headers.insert(
            "Content-type".to_string(), "text/html".to_string()
        );

        let expected_response = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: status_code.to_string(),
            status_text: "Not Found".to_string(),
            headers: Some(headers.clone()),
            body: Some(body.to_string()),
        };

        let response = HttpResponse::new(
            status_code,
            None,
            Some(body.to_string()),
        );
        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_send_response() {
        let status_code = "200";
        let body = "Item was shipped on 21st Dec 2020";
        let mut headers = HashMap::new();
        headers.insert(
            "Content-type".to_string(), "text/plain".to_string()
        );

        let response = HttpResponse::new(
            status_code,
            Some(headers.clone()),
            Some(body.to_string())
        );

        let mut output: Vec<u8> = Vec::new();
        response.send_response(&mut output).unwrap();
        let response_string = String::from_utf8(output).unwrap();

        let expected_response_str = format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            response.version(),
            response.status_code(),
            response.status_text(),
            response.headers(),
            response.body().len(),
            response.body()
        );

        assert_eq!(response_string, expected_response_str);
    }
}
