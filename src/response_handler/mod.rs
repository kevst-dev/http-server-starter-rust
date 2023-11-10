use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;

pub trait Handler {
    fn handle(request: &HttpRequest) -> HttpResponse;
}

mod path_not_found;
pub use path_not_found::PathNotFoundHandler;

mod echo;
pub use echo::EchoHandler;

mod path_default;
pub use path_default::PathDefaultHandler;

mod user_agent;
pub use user_agent::UserAgentHandler;
