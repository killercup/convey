//! JSON output

use serde::Serialize;
use serde_json::to_writer as write_json;
use std::io::Write;
use std::path::Path;
use {Error, Target};

/// Create a new JSON output that writes to a file
pub fn file<T: AsRef<Path>>(name: T) -> Result<Target, Error> {
    use std::fs::{File, OpenOptions};
    use std::io::BufWriter;

    let target = if name.as_ref().exists() {
        let mut f = OpenOptions::new().write(true).append(true).open(name)?;
        f.write_all(b"\n")?;

        f
    } else {
        File::create(name)?
    };

    let t = BufWriter::new(target);

    Ok(Target::Json(Formatter {
        writer: Box::new(t),
    }))
}

pub use self::test_helper::test;

/// JSON formatter
pub struct Formatter {
    pub(crate) writer: Box<Write>,
}

impl Formatter {
    /// Write a serializable item to the JSON formatter
    pub fn write<T: Serialize>(&mut self, item: &T) -> Result<(), Error> {
        write_json(&mut self.writer, item)?;
        self.writer.write_all(b"\n")?;
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

mod test_helper {
    use super::Formatter;
    use termcolor::Buffer;
    use test_buffer::TestBuffer;
    use Target;

    /// Create a test output target
    ///
    /// **Note:** This is intended for usage in tests; this and the methods you can call out in will
    /// often just panic instead of return `Result`s.
    ///
    /// # Usage
    ///
    /// ```rust
    /// #[macro_use] extern crate output;
    /// #[macro_use] extern crate serde_derive;
    ///
    /// fn main() -> Result<(), output::Error> {
    ///     let test_target = output::json::test();
    ///     let mut out = output::new().add_target(test_target.target());
    ///
    ///     #[derive(Serialize)]
    ///     struct Message {
    ///         author: String,
    ///         body: String,
    ///     }
    ///     impl output::Render for Message {
    ///         render_for_humans!(this -> []);
    ///         render_json!();
    ///     }
    ///
    ///     out.print(Message { author: "Pascal".into(), body: "Lorem ipsum dolor".into() })?;
    ///
    ///     assert_eq!(
    ///         test_target.to_string(),
    ///         "{\"author\":\"Pascal\",\"body\":\"Lorem ipsum dolor\"}\n\n",
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn test() -> TestTarget {
        TestTarget {
            buffer: Buffer::no_color().into(),
        }
    }

    pub struct TestTarget {
        buffer: TestBuffer,
    }

    impl TestTarget {
        pub fn formatter(&self) -> Formatter {
            Formatter {
                writer: Box::new(self.buffer.clone()),
            }
        }

        pub fn target(&self) -> Target {
            Target::Json(self.formatter())
        }

        pub fn to_string(&self) -> String {
            let target = self.buffer.0.clone();
            let buffer = target.read().unwrap();
            String::from_utf8_lossy(buffer.as_slice()).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::*;
    use json;
    use predicates::prelude::*;

    type Res = Result<(), ::failure::Error>;

    #[test]
    fn creates_a_new_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        log_file.assert(predicate::path::missing());

        {
            // drop to flush file write buffer
            let _target = json::file(log_file.path())?;
        }

        log_file.assert(predicate::path::exists());

        Ok(())
    }

    #[test]
    fn doesnt_truncate_existing_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        let intitial_content = "{\"success\":true}\n";

        log_file.write_str(intitial_content)?;
        log_file.assert(predicate::path::exists());

        {
            // drop to flush file write buffer
            let _target = json::file(log_file.path())?;
        }

        log_file.assert(
            predicate::str::contains(intitial_content)
                .from_utf8()
                .from_file_path(),
        );

        Ok(())
    }

    #[test]
    fn appends_to_existing_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        let intitial_content = "{\"success\":true}\n";

        log_file.write_str(intitial_content)?;
        log_file.assert(predicate::path::exists());

        {
            // drop to flush file write buffer
            let target = json::file(log_file.path())?;
            let mut output = ::new().add_target(target);
            output.print("wtf")?;
        }

        log_file.assert(
            predicate::str::ends_with("\"wtf\"")
                .trim()
                .from_utf8()
                .from_file_path(),
        );

        Ok(())
    }

    #[test]
    fn appends_newline_to_existing_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        let intitial_content = "{\"success\":true}"; // <- no newline at the end

        log_file.write_str(intitial_content)?;
        log_file.assert(predicate::path::exists());

        {
            // drop to flush file write buffer
            let _target = json::file(log_file.path())?;
        }

        log_file.assert(predicate::str::ends_with("\n").from_utf8().from_file_path());

        Ok(())
    }
}
