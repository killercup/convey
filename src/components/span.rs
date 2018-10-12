use termcolor::ColorSpec;
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
/// #[macro_use] extern crate convey;
///
/// fn main() -> Result<(), convey::Error> {
///     use convey::{components::text, human};
///
///     let mut out = convey::new().add_target(human::stdout()?);
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

    pub fn bold(mut self, yes: bool) -> Result<Self, Error> {
        self.bold = yes;
        Ok(self)
    }

    pub fn underline(mut self, yes: bool) -> Result<Self, Error> {
        self.underline = yes;
        Ok(self)
    }

    pub fn intense(mut self, yes: bool) -> Result<Self, Error> {
        self.intense = yes;
        Ok(self)
    }
}

impl Render for Span {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.set_color(
            ColorSpec::new()
                .set_fg(self.fg)
                .set_bg(self.bg)
                .set_bold(self.bold)
                .set_underline(self.underline)
                .set_intense(self.intense),
        )?;
        for item in &self.items {
            item.render_for_humans(fmt)?;
        }
        fmt.reset()?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        let len = self.items.len();
        for i in 0..len {
            self.items[i].render_json(fmt)?;
            if i < len - 1 {
                fmt.write_separator()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::span;
    use components::text;
    use {human, json, Error, Render};

    #[test]
    fn renders_span_children() -> Result<(), Error> {
        let item = span()
            .add_item(text("one"))
            .add_item(text("two"))
            .add_item(span().add_item(text("three")));

        let human_output = human::test();
        item.render_for_humans(&mut human_output.formatter())?;
        assert_eq!(&human_output.to_string(), "onetwothree");

        let json = json::test();
        item.render_json(&mut json.formatter())?;
        assert_eq!(json.to_string(), "\"one\"\n\"two\"\n\"three\"");
        Ok(())
    }

    #[test]
    fn test_colored_output() -> Result<(), Error> {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());

        out.print(span().add_item("hello").fg("green")?.bg("blue")?)?;
        out.flush()?;

        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[32m\u{1b}[44mhello\u{1b}[0m\n"
        );
        Ok(())
    }

    #[test]
    fn test_bold_output() -> Result<(), Error> {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());

        out.print(span().add_item("hello").bold(true)?)?;
        out.flush()?;

        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[1mhello\u{1b}[0m\n"
        );
        Ok(())
    }

    #[test]
    fn test_intense_output() -> Result<(), Error> {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());

        out.print(span().add_item("hello").fg("green")?.intense(true)?)?;
        out.flush()?;

        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[38;5;10mhello\u{1b}[0m\n"
        );
        Ok(())
    }

    #[test]
    fn test_underline_output() -> Result<(), Error> {
        let test_target = human::test_with_color();
        let mut out = ::new().add_target(test_target.target());

        out.print(span().add_item("hello").underline(true)?)?;
        out.flush()?;

        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[4mhello\u{1b}[0m\n"
        );
        Ok(())
    }

    // TODO: Add proptest tests
}
