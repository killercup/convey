use serde::Serialize;
use serde_json::to_writer as write_json;
use std::io::Write;
use std::path::Path;
use {Error, Target};

pub fn file<T: AsRef<Path>>(name: T) -> Result<Target, Error> {
    use std::fs::File;
    use std::io::BufWriter;
    let t = BufWriter::new(File::create(name)?);

    Ok(Target::Json(Formatter {
        writer: Box::new(t),
    }))
}

pub struct Formatter {
    pub(crate) writer: Box<Write>,
}

impl Formatter {
    pub fn write<T: Serialize>(&mut self, item: &T) -> Result<(), Error> {
        write_json(&mut self.writer, item)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! render_json {
    () => {
        fn render_json(&self, fmt: &mut $crate::json::Formatter) -> Result<(), $crate::Error> {
            fmt.write(self)?;
            Ok(())
        }
    }
}
