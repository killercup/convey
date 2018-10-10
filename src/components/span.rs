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
    bold: bool,
    underline: bool,
    intense: bool,
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

    pub fn bold(mut self, yes: bool) -> Self {
        self.bold = yes;
        self
    }

    pub fn underline(mut self, yes: bool) -> Self {
        self.underline = yes;
        self
    }

    pub fn intense(mut self, yes: bool) -> Self {
        self.intense = yes;
        self
    }
}

impl Render for Span {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.writer.set_color(
            ColorSpec::new()
                .set_fg(self.fg)
                .set_bg(self.bg)
                .set_bold(self.bold)
                .set_underline(self.underline),
        )?;
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

#[cfg(test)]
mod test {
    use super::span;
    use components::text;
    use {human, json, Render};

    #[test]
    fn renders_span_children() {
        let item = span()
            .add_item(text("one"))
            .add_item(text("two"))
            .add_item(span().add_item(text("three")));

        let human_output = human::test();
        item.render_for_humans(&mut human_output.formatter())
            .unwrap();
        assert_eq!(&human_output.to_string(), "onetwothree");

        let json = json::test();
        item.render_json(&mut json.formatter()).unwrap();
        assert_eq!(json.to_string(), "\"one\"\n\"two\"\n\"three\"\n");
    }

    #[test]
    fn test_colored_output() {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());
        out.print(
            span()
                .add_item("hello")
                .fg("green")
                .unwrap()
                .bg("blue")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[32m\u{1b}[44mhello\u{1b}[0m\n"
        )
    }

    #[test]
    fn test_bold_output() {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());
        out.print(span().add_item("hello").bold(true)).unwrap();
        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[1mhello\u{1b}[0m\n"
        )
    }

    #[test]
    fn test_intense_output() {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());
        out.print(span().add_item("hello").intense(true)).unwrap();
        assert_eq!(test_target.to_string(), "\u{1b}[0mhello\u{1b}[0m\n")
    }

    #[test]
    fn test_underline_output() {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());
        out.print(span().add_item("hello").underline(true)).unwrap();
        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[4mhello\u{1b}[0m\n"
        )
    }

    // TODO: Add proptest tests
}
