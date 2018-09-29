use std::io;
use termcolor::{ColorSpec, WriteColor};

pub use termcolor::Buffer as TestOutput;

pub trait TerminalOutput {
    type Handle;

    fn output(&self, f: &mut TerminalFormatter) -> Result<Self::Handle, TerminalFormatterError>;
}

#[derive(Debug)]
pub enum TerminalFormatterError {
    Io(io::Error),
}

impl From<io::Error> for TerminalFormatterError {
    fn from(e: io::Error) -> Self {
        TerminalFormatterError::Io(e)
    }
}

pub trait Target: io::Write + WriteColor {}
impl<T> Target for T where T: io::Write + WriteColor {}

pub struct TerminalFormatter<'a> {
    target: Box<dyn Target + 'a>,
    supports_color: bool,
}

impl<'a> TerminalFormatter<'a> {
    pub fn new<T: Target + 'a>(target: T) -> Self {
        TerminalFormatter {
            target: Box::new(target),
            supports_color: false,
        }
    }

    pub fn color(&mut self, supported: bool) {
        self.supports_color = supported;
    }
}

impl<'a> io::Write for TerminalFormatter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.target.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.target.flush()
    }
}

impl<'a> WriteColor for TerminalFormatter<'a> {
    fn supports_color(&self) -> bool {
        self.supports_color
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.target.set_color(spec)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.target.reset()
    }
}
