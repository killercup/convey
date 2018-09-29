extern crate termcolor;
use std::io::Write;
use termcolor::WriteColor;

mod fmt;
use fmt::{TerminalFormatter, TerminalFormatterError, TerminalOutput};

struct Text(String);

impl TerminalOutput for Text {
    type Handle = ();

    fn output(&self, f: &mut TerminalFormatter) -> Result<(), TerminalFormatterError> {
        f.write(self.0.as_bytes())?;
        Ok(())
    }
}

// fixme(killercup): how to deal with this associated type here?
// refactor trait to return a known (dynamic) type instead?
struct Color(
    Vec<Box<dyn TerminalOutput<Handle = ()>>>,
    termcolor::ColorSpec,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_bytes_as_str(left: &[u8], right: &[u8]) {
        let left = String::from_utf8(left.to_vec()).unwrap();
        let right = String::from_utf8(right.to_vec()).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn write_text() {
        let mut out = fmt::TestOutput::no_color();

        {
            let mut fmt = TerminalFormatter::new(&mut out);
            let x = Text(String::from("foo"));
            x.output(&mut fmt).unwrap();
        }

        assert_bytes_as_str(out.as_slice(), b"foo");
    }

    #[test]
    fn write_color() {
        let mut out = fmt::TestOutput::ansi();

        {
            let mut fmt = TerminalFormatter::new(&mut out);
            let x = Color(vec![Box::new(Text(String::from("foo")))], {
                use termcolor::{Color, ColorSpec};
                let mut c = ColorSpec::new();
                c.set_fg(Some(Color::Blue));
                c
            });
            x.output(&mut fmt).unwrap();
        }

        assert_bytes_as_str(out.as_slice(), "\u{1b}[0m\u{1b}[34mfoo\u{1b}[0m".as_bytes());
    }
}
