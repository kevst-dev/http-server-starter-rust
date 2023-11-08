use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    // ---- -- ParseUrl -- ---- //
    #[error("No se pudo parsear la URL: {0}.")]
    UrlParseError(String),
}
