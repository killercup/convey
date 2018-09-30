use std::io;
use serde::Serialize;
use serde_json::{error::Error as JsonError, to_writer as write_json};
use std::path::Path;

pub trait Output<'f> {
    fn json_output(&self, fmt: &mut Formatter<'f>) -> Result<(), Error>;
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error),
    #[fail(display = "Error while serializing: {}", _0)]
    Serialize(JsonError),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::Serialize(e)
    }
}

/// JSON output format
pub struct Formatter<'a> {
    target: Box<dyn io::Write + 'a>,
}

impl<'a> Formatter<'a> {
    fn new<T: io::Write + 'a>(target: T) -> Self {
        Self {
            target: Box::new(target),
        }
    }

    pub fn file<T: AsRef<Path>>(name: T) -> Result<Self, Error> {
        use std::io::BufWriter;
        use std::fs::File;
        let t = BufWriter::new(File::create(name)?);

        Ok(Self {
            target: Box::new(t),
        })
    }

    pub fn write<T: Serialize>(&mut self, doc: &T) -> Result<(), Error> {
        write_json(&mut self.target, doc)?;
        Ok(())
    }
}
