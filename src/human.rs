//! Human output

use termcolor::{ColorChoice, StandardStream, WriteColor};
use {Error, Target};

/// Construct a new human output target that writes to stdout
pub fn stdout() -> Result<Target, Error> {
    Ok(Target::Human(Formatter {
        writer: Box::new(StandardStream::stdout(ColorChoice::Auto)),
    }))
}

pub use self::test_helper::{test, test_with_color};

/// Human output formatter
pub struct Formatter {
    pub(crate) writer: Box<dyn WriteColor>,
}

/// Shorthand for writing the `render_for_humans` method of the `Render`  trait
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate output;
/// #[macro_use] extern crate serde_derive;
///
/// use output::{components::{text, newline}, Render};
///
/// #[derive(Serialize)]
/// struct Message {
///     author: String,
///     body: String,
/// }
///
/// impl Render for Message {
///     // because `self` is a keyword, we need to use something else
///     render_for_humans!(self -> [
///         // compose output components
///         text("Important notice from "),
///         // refer to struct fields using the name you specified above
///         text(&self.author), newline(),
///         text("> "), text(&self.body),
///     ]);
///
///     // see `json` module
///     render_json!();
/// }
///
/// fn main() -> Result<(), output::Error> {
///     let mut out = output::new().add_target(output::human::stdout()?);
///     out.print(Message { author: "Pascal".into(), body: "Lorem ipsum dolor".into() })?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! render_for_humans {
    ($this:ident -> []) => {
        fn render_for_humans(&self, fmt: &mut $crate::human::Formatter) -> Result<(), $crate::Error> {
            Ok(())
        }
    };
    ($self:ident -> [$($item:expr,)*]) => {
        fn render_for_humans(&$self, fmt: &mut $crate::human::Formatter) -> Result<(), $crate::Error> {
            let span = span!([ $( $item, )* ]);
            span.render_for_humans(fmt)?;
            Ok(())
        }
    };
}

mod test_helper {
    use super::Formatter;
    use termcolor::Buffer;
    use {test_buffer::TestBuffer, Target};

    /// Create a test output target
    ///
    /// This will drop all color information! If you want test colored output, use
    /// [`test_with_color`] instead.
    ///
    /// **Note:** This is intended for usage in tests; this and the methods you can call out in will
    /// often just panic instead of return `Result`s.
    ///
    /// # Usage
    ///
    /// ```rust
    /// extern crate output;
    ///
    /// fn main() -> Result<(), output::Error> {
    ///     let test_target = output::human::test();
    ///     let mut out = output::new().add_target(test_target.target());
    ///     out.print(output::components::text("lorem ipsum"))?;
    ///
    ///     assert_eq!(test_target.to_string(), "lorem ipsum\n");
    ///     Ok(())
    /// }
    /// ```
    pub fn test() -> TestTarget {
        TestTarget {
            buffer: Buffer::no_color().into(),
        }
    }

    /// Create a test out target that also contains ANSI color codes
    ///
    /// For usage, see [`test()`].
    pub fn test_with_color() -> TestTarget {
        TestTarget {
            buffer: Buffer::ansi().into(),
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
            Target::Human(self.formatter())
        }

        pub fn to_string(&self) -> String {
            let target = self.buffer.0.clone();
            let buffer = target.read().unwrap();
            String::from_utf8_lossy(buffer.as_slice()).to_string()
        }
    }
}
