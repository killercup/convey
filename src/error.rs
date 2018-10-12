use serde_json::Error as JsonError;
use std::io;
use termcolor::ParseColorError;

/// Output's error type
#[derive(Fail, Debug)]
pub enum Error {
    /// I/O Error
    #[fail(display = "IO error: {}", _0)]
    Io(io::Error),
    /// Error parsing a color value
    #[fail(display = "{}", _0)]
    ParseColorError(ParseColorError),
    /// Error dealing with JSON
    #[fail(display = "{}", _0)]
    Json(JsonError),
    /// Error in formatting worker
    #[fail(display = "{}", _0)]
    WorkerError(String),
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error::Io(x)
    }
}

impl From<ParseColorError> for Error {
    fn from(x: ParseColorError) -> Self {
        Error::ParseColorError(x)
    }
}

impl From<JsonError> for Error {
    fn from(x: JsonError) -> Self {
        Error::Json(x)
    }
}
