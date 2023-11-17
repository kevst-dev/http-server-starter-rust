use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;

pub trait Handler<OptionalPath> {
    fn handle(request: &HttpRequest, path: OptionalPath) -> HttpResponse;
}

mod path_not_found;
pub use path_not_found::PathNotFoundHandler;

mod echo;
pub use echo::EchoHandler;

mod path_default;
pub use path_default::PathDefaultHandler;

mod user_agent;
pub use user_agent::UserAgentHandler;

mod get_file;
pub use get_file::GetFileHandler;

mod post_file;
pub use post_file::PostFileHandler;
