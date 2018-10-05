//! JSON output

use serde::Serialize;
use serde_json::to_writer as write_json;
use std::io::Write;
use std::path::Path;
use {Error, Target};

/// Create a new JSON output that writes to a file
pub fn file<T: AsRef<Path>>(name: T) -> Result<Target, Error> {
    use std::fs::File;
    use std::io::BufWriter;
    let t = BufWriter::new(File::create(name)?);

    Ok(Target::Json(Formatter {
        writer: Box::new(t),
    }))
}

/// JSON formatter
pub struct Formatter {
    pub(crate) writer: Box<Write>,
}

impl Formatter {
    /// Write a serializable item to the JSON formatter
    pub fn write<T: Serialize>(&mut self, item: &T) -> Result<(), Error> {
        write_json(&mut self.writer, item)?;
        Ok(())
    }
}

/// Shorthand for writing the `render_json` method of the `Render`  trait
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate output;
/// #[macro_use] extern crate serde_derive;
///
/// #[derive(Serialize)]
/// struct Message {
///     author: String,
///     body: String,
/// }
///
/// impl output::Render for Message {
///     // Look at the `human` module if you care about those meat bags.
///     render_for_humans!(this -> []);
///
///     // We're lucky, or type implements `Serialize`. Nothing to do!
///     render_json!();
/// }
///
/// fn main() -> Result<(), output::Error> {
///     let mut out = output::new().add_target(output::human::stdout()?);
///     out.print(Message { author: "Pascal".into(), body: "Lorem ipsum dolor".into() })?;
///     Ok(())
/// }
/// ```
// TODO: Add mode for stuff like `render_json!(this -> { "success": true, "excerpt": this.body.lines().next() });`
#[macro_export]
macro_rules! render_json {
    () => {
        fn render_json(&self, fmt: &mut $crate::json::Formatter) -> Result<(), $crate::Error> {
            fmt.write(self)?;
            Ok(())
        }
    }
}
