extern crate termcolor;

use std::io::Write;
use termcolor::WriteColor;

mod fmt;
pub(crate) use fmt::{TerminalFormatter, TerminalFormatterError, TerminalOutput};

mod components;
pub use components::color::Color;
pub use components::text::Text;

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
