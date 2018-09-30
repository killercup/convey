use std::io;
use termcolor::{ColorSpec, WriteColor, StandardStream};

pub trait Output<'f> {
    fn human_output(&self, fmt: &mut Formatter<'f>) -> Result<(), Error>;
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub trait Target: io::Write + WriteColor {}
impl<T> Target for T where T: io::Write + WriteColor {}

pub struct Formatter<'a> {
    target: Box<dyn Target + 'a>,
    supports_color: bool,
}

impl<'a> Formatter<'a> {
    fn new<T: Target + 'a>(target: T) -> Self {
        Self {
            target: Box::new(target),
            supports_color: false,
        }
    }

    pub fn stdout() -> Result<Self, Error> {
        use termcolor::ColorChoice;
        let t = StandardStream::stdout(ColorChoice::Auto);

        Ok(Self {
            target: Box::new(t),
            supports_color: false,
        })
    }

    pub fn color(&mut self, supported: bool) {
        self.supports_color = supported;
    }
}

impl<'a> io::Write for Formatter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.target.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.target.flush()
    }
}

impl<'a> WriteColor for Formatter<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;
    pub use termcolor::Buffer as TestOutput;
    use Color;
    use Text;

    fn assert_bytes_as_str(left: &[u8], right: &[u8]) {
        let left = String::from_utf8(left.to_vec()).unwrap();
        let right = String::from_utf8(right.to_vec()).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn write_text() {
        let mut out = TestOutput::no_color();

        {
            let mut fmt = Formatter::new(&mut out);
            let x = Text(String::from("foo"));
            x.human_output(&mut fmt).unwrap();
        }

        assert_bytes_as_str(out.as_slice(), b"foo");
    }

    #[test]
    fn write_color() {
        let mut out = TestOutput::ansi();

        {
            let mut fmt = Formatter::new(&mut out);
            let x = Color(vec![Box::new(Text(String::from("foo")))], {
                use termcolor::{Color, ColorSpec};
                let mut c = ColorSpec::new();
                c.set_fg(Some(Color::Blue));
                c
            });
            x.human_output(&mut fmt).unwrap();
        }

        assert_bytes_as_str(out.as_slice(), "\u{1b}[0m\u{1b}[34mfoo\u{1b}[0m".as_bytes());
    }
}