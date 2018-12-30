use failure::{Backtrace, Context, Fail};
use serde_json::Error as JsonError;
use std::fmt::{self, Display};
use std::io;
use std::sync::PoisonError;
use termcolor::ParseColorError;

#[derive(Debug)]
/// Output's error type
pub struct Error {
    inner: Context<InnerError>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

#[derive(Fail, Debug)]
enum InnerError {
    #[fail(display = "IO error: {}", _0)]
    Io(io::Error),

    #[fail(display = "{}", _0)]
    ParseColorError(ParseColorError),

    #[fail(display = "Json error: {}", _0)]
    Json(JsonError),

    #[fail(display = "Worker error: {}", _0)]
    WorkerError(String),

    #[fail(display = "Error syncing output")]
    SyncError(String),

    #[fail(display = "Error sending data to channel")]
    ChannelError(String),

    #[fail(display = "{}", _0)]
    SetLoggerError(log::SetLoggerError),
}

impl Error {
    pub(crate) fn worker_error(x: String) -> Self {
        Error {
            inner: Context::new(InnerError::WorkerError(x)),
        }
    }

    pub(crate) fn sync_error<T>(x: &PoisonError<T>) -> Self {
        Error {
            inner: Context::new(InnerError::SyncError(x.to_string())),
        }
    }
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Self {
        Error {
            inner: Context::new(InnerError::Io(x)),
        }
    }
}

impl From<ParseColorError> for Error {
    fn from(x: ParseColorError) -> Self {
        Error {
            inner: Context::new(InnerError::ParseColorError(x)),
        }
    }
}

impl From<JsonError> for Error {
    fn from(x: JsonError) -> Self {
        Error {
            inner: Context::new(InnerError::Json(x)),
        }
    }
}

impl<T: std::fmt::Debug> From<crossbeam_channel::SendError<T>> for Error {
    fn from(x: crossbeam_channel::SendError<T>) -> Self {
        Error {
            inner: Context::new(InnerError::ChannelError(x.to_string())),
        }
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(x: log::SetLoggerError) -> Self {
        Error {
            inner: Context::new(InnerError::SetLoggerError(x)),
        }
    }
}
