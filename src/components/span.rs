use termcolor::{ColorSpec, WriteColor};
use {human, json, Error, Render};

/// Construct a new, empty span
pub fn span() -> Span {
    Span::default()
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_span {
    ($span:ident, $attr:ident = $val:expr, $($token:tt)*) => {
        $span = $span.$attr($val)?;
        __inner_span!($span, $($token)*);
    };
    ($span:ident, [$($item:expr,)*]) => {
        $(
            $span = $span.add_item($item);
        )*
    };
    ($span:ident, []) => { };
}

/// Quickly write a span
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate output;
///
/// fn main() -> Result<(), output::Error> {
///     use output::{components::text, human};
///
///     let mut out = output::new().add_target(human::stdout()?);
///
///     let message = span!(fg = "red", [
///         text("hello"),
///     ]);
///
///     out.print(message)?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! span {
    ($($token:tt)*) => {
        {
            let mut the_span = $crate::components::span();
            __inner_span!(the_span, $($token)*);
            the_span
        }
    };
}

#[derive(Default)]
pub struct Span {
    items: Vec<Box<Render>>,
    fg: Option<::termcolor::Color>,
    bg: Option<::termcolor::Color>,
}

impl Span {
    pub fn add_item<T: Render + 'static>(mut self, item: T) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn fg(mut self, color: &str) -> Result<Self, Error> {
        self.fg = Some(color.parse()?);
        Ok(self)
    }

    pub fn bg(mut self, color: &str) -> Result<Self, Error> {
        self.bg = Some(color.parse()?);
        Ok(self)
    }
}

impl Render for Span {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.writer
            .set_color(ColorSpec::new().set_fg(self.fg).set_bg(self.bg))?;
        for item in &self.items {
            item.render_for_humans(fmt)?;
        }
        fmt.writer.reset()?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        for item in &self.items {
            item.render_json(fmt)?;
        }
        Ok(())
    }
}
