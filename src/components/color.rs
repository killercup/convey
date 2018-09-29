use std::io::Write;
use termcolor::WriteColor;

use {TerminalOutput, TerminalFormatter, TerminalFormatterError};
use Text;

// fixme(killercup): how to deal with this associated type here?
// refactor trait to return a known (dynamic) type instead?
pub struct Color(
    pub Vec<Box<dyn TerminalOutput<Handle = ()>>>,
    pub termcolor::ColorSpec,
);

impl TerminalOutput for Color {
    type Handle = ();

    fn output(&self, f: &mut TerminalFormatter) -> Result<(), TerminalFormatterError> {
        f.set_color(&self.1)?;
        self.0.iter().try_for_each(|x| x.output(f))?;
        f.reset()?;
        Ok(())
    }
}