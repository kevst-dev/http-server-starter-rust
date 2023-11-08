use std::fmt;
use std::str::FromStr;

use crate::errors::Errors;

#[derive(Debug, PartialEq)]
pub struct ParseUrl {
    pub host: String,
    pub port: String,
    pub path: String,
}

impl ParseUrl {
    pub fn new(host: &str, port: &str, path: &str) -> ParseUrl {
        ParseUrl {
            host: host.to_string(),
            port: port.to_string(),
            path: path.to_string(),
        }
    }

    pub fn get_host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl FromStr for ParseUrl {
    type Err = Errors;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let split_url: Vec<&str> = url.split(':').collect();

        if split_url.len() != 2 {
            return Err(Errors::UrlParseError(url.to_string()));
        }

        let host = split_url[0];
        let split_url: Vec<&str> = split_url[1].split('/').collect();

        if split_url.len() != 2 {
            return Err(Errors::UrlParseError(url.to_string()));
        }

        let port = split_url[0];
        let path = split_url[1];

        Ok(ParseUrl::new(host, port, path))
    }
}

impl fmt::Display for ParseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{host}:{port}/{path}",
            host = self.host,
            port = self.port,
            path = self.path
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // ---- -- FromStr -- ---- \\

    #[test]
    fn test_from_str_valid_with_path() {
        let expected_url = ParseUrl::new("192.168.0.1", "8080", "index.html");
        let url = "192.168.0.1:8080/index.html";

        let result_url = ParseUrl::from_str(url).unwrap();

        assert_eq!(result_url, expected_url);
    }

    #[test]
    fn test_from_str_valid_without_path() {
        let expected_url = ParseUrl::new("192.168.0.1", "8080", "");
        let url = "192.168.0.1:8080/";

        let result_url = ParseUrl::from_str(url).unwrap();

        assert_eq!(result_url, expected_url);
    }

    #[test]
    fn test_from_str_invalid_host() {
        let url = "192.168.0.1-8080/";

        let err = ParseUrl::from_str(url).unwrap_err();

        assert!(matches!(err, Errors::UrlParseError(_)));
    }

    #[test]
    fn test_from_str_invalid_path() {
        let url = "192.168.0.1:8080-index.html";

        let err = ParseUrl::from_str(url).unwrap_err();

        assert!(matches!(err, Errors::UrlParseError(_)));
    }

    // ---- -- Display -- ---- \\

    #[test]
    fn test_display_with_path() {
        let expected_url = "192.168.0.1:8080/index.html";
        let url = ParseUrl::new("192.168.0.1", "8080", "index.html");

        assert_eq!(url.to_string(), expected_url)
    }

    #[test]
    fn test_display_without_path() {
        let expected_url = "192.168.0.1:8080/";
        let url = ParseUrl::new("192.168.0.1", "8080", "");

        assert_eq!(url.to_string(), expected_url)
    }
}
