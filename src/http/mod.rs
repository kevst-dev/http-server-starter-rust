mod parser;
pub use parser::parse_request;

pub const SUPPORTED_ENCODEING: [&str; 1] = ["gzip"];

mod uri_path;
pub use uri_path::UriPath;

mod request;
pub use request::{HttpMethod, HttpRequest, HttpVersion, RequestLine};

mod response;
pub use response::HttpResponse;
