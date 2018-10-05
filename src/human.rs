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
///     render_for_humans!(this -> [
///         // compose output components
///         text("Important notice from "),
///         // refer to struct fields using the name you specified above
///         text(&this.author), newline(),
///         text("> "), text(&this.body),
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
    ($this:ident -> [$($item:expr,)*]) => {
        fn render_for_humans(&self, fmt: &mut $crate::human::Formatter) -> Result<(), $crate::Error> {
            let $this = self;
            let span = span!([ $( $item, )* ]);
            span.render_for_humans(fmt)?;
            Ok(())
        }
    };
}

mod test_helper {
    use super::Formatter;
    use std::io;
    use std::sync::{Arc, RwLock};
    use termcolor::{Buffer, ColorSpec, WriteColor};
    use {Error, Target};

    /// Create a test output target
    ///
    /// This will drop all color information! If you want test colored output, use
    /// [`test_with_color`] instead.
    ///
    /// # Usage
    ///
    /// ```rust
    /// extern crate output;
    ///
    /// fn main() -> Result<(), output::Error> {
    ///     let test_target = output::human::test()?;
    ///     let mut out = output::new().add_target(test_target.target()?);
    ///     out.print(output::components::text("lorem ipsum"))?;
    ///
    ///     assert_eq!(test_target.as_string(), "lorem ipsum\n");
    ///     Ok(())
    /// }
    /// ```
    pub fn test() -> Result<TestTarget, Error> {
        Ok(TestTarget::new(Buffer::no_color()))
    }

    /// Create a test out target that also contains ANSI color codes
    ///
    /// For usage, see [`test()`].
    pub fn test_with_color() -> Result<TestTarget, Error> {
        Ok(TestTarget::new(Buffer::ansi()))
    }

    pub struct TestTarget {
        buffer: TestBuffer,
    }

    #[derive(Clone)]
    struct TestBuffer(Arc<RwLock<Buffer>>);

    impl TestTarget {
        fn new(buffer: Buffer) -> Self {
            TestTarget {
                buffer: TestBuffer(Arc::new(RwLock::new(buffer))),
            }
        }

        pub fn target(&self) -> Result<Target, Error> {
            Ok(Target::Human(Formatter {
                writer: Box::new(self.buffer.clone()),
            }))
        }

        pub fn as_string(&self) -> String {
            let target = self.buffer.0.clone();
            let buffer = target.read().unwrap();
            String::from_utf8_lossy(buffer.as_slice()).to_string()
        }
    }

    impl io::Write for TestBuffer {
        fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
            let target = self.0.clone();
            let mut buffer = target.write().unwrap();
            buffer.write(buf)
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            let target = self.0.clone();
            let mut buffer = target.write().unwrap();
            buffer.flush()
        }
    }

    impl WriteColor for TestBuffer {
        fn supports_color(&self) -> bool {
            let target = self.0.clone();
            let buffer = target.read().unwrap();
            buffer.supports_color()
        }

        fn set_color(&mut self, spec: &ColorSpec) -> Result<(), io::Error> {
            let target = self.0.clone();
            let mut buffer = target.write().unwrap();
            buffer.set_color(spec)
        }

        fn reset(&mut self) -> Result<(), io::Error> {
            let target = self.0.clone();
            let mut buffer = target.write().unwrap();
            buffer.reset()
        }
    }
}
