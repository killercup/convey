use std::io;
use termcolor::{ColorChoice, StandardStream};
use Target;

pub fn stdout() -> Result<Target, io::Error> {
    Ok(Target::Human(Formatter {
        writer: StandardStream::stdout(ColorChoice::Auto),
    }))
}

pub struct Formatter {
    pub(crate) writer: StandardStream,
}
