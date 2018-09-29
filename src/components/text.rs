use std::io::Write;

use {TerminalOutput, TerminalFormatter, TerminalFormatterError};

pub struct Text(pub String);

impl TerminalOutput for Text {
    type Handle = ();

    fn output(&self, f: &mut TerminalFormatter) -> Result<(), TerminalFormatterError> {
        f.write(self.0.as_bytes())?;
        Ok(())
    }
}