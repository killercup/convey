use std::io;

trait TerminalOutput {
    type Handle;

    fn output(&self, f: &mut TerminalFormatter) -> Result<Self::Handle, TerminalFormatterError>;
}

#[derive(Debug)]
enum TerminalFormatterError {
    Io(io::Error),
}

impl From<io::Error> for TerminalFormatterError {
    fn from(e: io::Error) -> Self {
        TerminalFormatterError::Io(e)
    }
}

struct TerminalFormatter<'a> {
    target: Box<io::Write + 'a>,
}

impl<'a> TerminalFormatter<'a> {
    fn new<T: io::Write + 'a>(target: T) -> Self {
        TerminalFormatter {
            target: Box::new(target),
        }
    }

    fn write(&mut self, c: &[u8]) -> Result<(), TerminalFormatterError> {
        self.target.write_all(c)?;
        Ok(())
    }
}

struct Text(String);

impl TerminalOutput for Text {
    type Handle = ();

    fn output(&self, f: &mut TerminalFormatter) -> Result<(), TerminalFormatterError> {
        f.write(self.0.as_bytes())?;
        Ok(())
    }
}

#[test]
fn write_stuff() {
    let mut out = Vec::new();

    {
        let mut fmt = TerminalFormatter::new(&mut out);
        let x = Text(String::from("foo"));
        x.output(&mut fmt).unwrap();
    }

    assert_eq!(out, b"foo");
}
