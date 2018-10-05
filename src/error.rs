use std::io;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    Io(io::Error),
    #[fail(display = "{}", _0)]
    ParseColorError(termcolor::ParseColorError),
    #[fail(display = "{}", _0)]
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error::Io(x)
    }
}

impl From<termcolor::ParseColorError> for Error {
    fn from(x: termcolor::ParseColorError) -> Self {
        Error::ParseColorError(x)
    }
}

impl From<serde_json::Error> for Error {
    fn from(x: serde_json::Error) -> Self {
        Error::Json(x)
    }
}
