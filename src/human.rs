//! Human output

use std::io;
use termcolor::{ColorChoice, StandardStream};
use Target;

/// Construct a new human output target that writes to stdout
pub fn stdout() -> Result<Target, io::Error> {
    Ok(Target::Human(Formatter {
        writer: StandardStream::stdout(ColorChoice::Auto),
    }))
}

/// Human output formatter
pub struct Formatter {
    pub(crate) writer: StandardStream,
}

/// Shorthand for writing the `render_for_humans` method of the `RenderOutput`  trait
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate output;
/// #[macro_use] extern crate serde_derive;
///
/// use output::{components::{text, newline}, RenderOutput};
///
/// #[derive(Serialize)]
/// struct Message {
///     author: String,
///     body: String,
/// }
///
/// impl RenderOutput for Message {
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
