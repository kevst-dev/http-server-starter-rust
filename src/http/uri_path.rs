use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct UriPath {
    path: String,
    data: String,
}

impl UriPath {
    pub fn new(path: &str) -> Self {
        let mut iter = path.splitn(3, '/');

        let _ = iter.next().unwrap_or("").to_string();
        let path = iter.next().unwrap_or("").to_string();
        let data = iter.next().unwrap_or("").to_string();

        UriPath { path, data }
    }

    //  function name starts_with is more semantic
    pub fn path(&self) -> String {
        if self.path.is_empty() {
            return "/".to_string();
        };

        format!("/{}", self.path)
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}

impl fmt::Display for UriPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = if self.data.is_empty() {
            "".to_string()
        } else {
            format!("/{}", self.data)
        };

        write!(f, "{}{}", self.path(), data,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_path() {
        let url = "/";
        let path = UriPath::new(url);

        assert_eq!(path.path(), "/");
        assert_eq!(path.data(), "");
        assert_eq!(path.to_string(), url);
    }

    #[test]
    fn test_complex_path() {
        let url = "/echo/linux";
        let path = UriPath::new(url);

        assert_eq!(path.path(), "/echo");
        assert_eq!(path.data(), "linux");
        assert_eq!(path.to_string(), url);
    }

    #[test]
    fn test_complex_path_with_slashes() {
        let url = "/echo/monkey/Coo-donkey";
        let path = UriPath::new(url);

        assert_eq!(path.path(), "/echo");
        assert_eq!(path.data(), "monkey/Coo-donkey");
        assert_eq!(path.to_string(), url);
    }

    #[test]
    fn test_complex_path_with_slashes_1() {
        let url = "/echo/Coo/dooby";
        let path = UriPath::new(url);

        assert_eq!(path.path(), "/echo");
        assert_eq!(path.data(), "Coo/dooby");
        assert_eq!(path.to_string(), url);
    }
}
