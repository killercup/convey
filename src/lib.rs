extern crate failure;
extern crate termcolor;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub fn new() -> Output {
    Output::default()
}

#[derive(Default)]
pub struct Output {
    targets: Vec<Target>,
}

impl Output {
    pub fn add(mut self, target: Target) -> Self {
        self.targets.push(target);
        self
    }
}

pub enum Target {
    Human(human::Formatter),
    Json(json::Formatter),
}

impl Output {
    pub fn print<O: RenderOutput>(&mut self, item: O) -> Result<(), ::failure::Error> {
        for target in &mut self.targets {
            match target {
                Target::Human(fmt) => item.render_for_humans(fmt)?,
                Target::Json(fmt) => item.render_json(fmt)?,
            }
        }

        Ok(())
    }
}

pub trait RenderOutput {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), ::failure::Error>;
    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), ::failure::Error>;
}

impl<'a, T> RenderOutput for &'a T where T: RenderOutput {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), ::failure::Error> {
        (*self).render_for_humans(fmt)
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), ::failure::Error> {
        (*self).render_json(fmt)
    }
}

pub mod human {
    use std::io;
    use termcolor::{StandardStream, ColorChoice};
    use Target;

    pub fn stdout() -> Result<Target, io::Error> {
        Ok(Target::Human(Formatter {
            writer: StandardStream::stdout(ColorChoice::Auto),
        }))
    }

    pub struct Formatter {
        pub(crate) writer: StandardStream,
    }
}

pub mod json {
    use std::path::Path;
    use std::io::Write;
    use failure::Error;
    use serde::Serialize;
    use serde_json::to_writer as write_json;
    use Target;

    pub fn file<T: AsRef<Path>>(name: T) -> Result<Target, Error> {
        use std::io::BufWriter;
        use std::fs::File;
        let t = BufWriter::new(File::create(name)?);

        Ok(Target::Json(Formatter {
            writer: Box::new(t),
        }))
    }

    pub struct Formatter {
        writer: Box<Write>,
    }

    impl Formatter {
        pub fn write<T: Serialize>(&mut self, item: &T) -> Result<(), Error> {
            write_json(&mut self.writer, item)?;
            Ok(())
        }
    }
}

pub mod components {
    use std::io::Write;
    use failure::Error;
    use {RenderOutput, human, json};

    pub fn text<T: Into<String>>(input: T) -> Text {
        Text(input.into())
    }

    #[derive(Clone, Serialize)]
    pub struct Text(String);

    impl RenderOutput for Text {
        fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
            fmt.writer.write(self.0.as_bytes())?;
            Ok(())
        }

        fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
            fmt.write(self)?;
            Ok(())
        }
    }
}
