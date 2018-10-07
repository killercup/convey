//! Easily output stuff for humans and machines alike
//!
//! # Examples
//!
//! ```rust
//! extern crate output;
//!
//! fn main() -> Result<(), output::Error> {
//!     let mut out = output::new().add_target(output::human::stdout()?);
//!     out.print(output::components::text("hello world!"))?;
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
extern crate termcolor;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate proptest;

use std::io::Write;

/// Create a new output
pub fn new() -> Output {
    Output::default()
}

/// Structure holding your output targets
#[derive(Default)]
pub struct Output {
    targets: Vec<Target>,
}

impl Output {
    /// Add a target to output to
    pub fn add_target(mut self, target: Target) -> Self {
        self.targets.push(target);
        self
    }
}

/// Known targets to write to
pub enum Target {
    /// Human readable output
    ///
    /// Will mostly be (unstructured) text, optionally with formatting.
    Human(human::Formatter),
    /// JSON output
    ///
    /// Machines like this.
    Json(json::Formatter),
}

mod error;
pub use error::Error;

impl Output {
    /// Print some item to the currently active output targets
    pub fn print<O: Render>(&mut self, item: O) -> Result<(), Error> {
        for target in &mut self.targets {
            match target {
                Target::Human(fmt) => {
                    item.render_for_humans(fmt)?;
                    fmt.writer.write_all(b"\n")?;
                }
                Target::Json(fmt) => {
                    item.render_json(fmt)?;
                    fmt.writer.write_all(b"\n")?;
                }
            }
        }

        Ok(())
    }
}

/// Implement this for your own components
pub trait Render {
    /// How to render your type for humans
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error>;
    /// How to render your type to JSON
    ///
    /// If your type implements `Serialize`, this can easily just be
    /// `fmt.write(self)`. Alternatively, you might want to use something like
    /// serde_json's `json!` macro.
    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error>;
}

impl<'a, T> Render for &'a T
where
    T: Render,
{
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        (*self).render_for_humans(fmt)
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        (*self).render_json(fmt)
    }
}

impl<'a> Render for &'a str {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.writer.write_all(self.as_bytes())?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        fmt.write(&self)?;
        Ok(())
    }
}

pub mod components;
pub mod human;
pub mod json;

mod test_buffer;

#[cfg(test)]
mod tests {
    use components::span;
    use human;

    #[test]
    fn test_print_str() {
        let test_target = human::test();
        let mut out = ::new().add_target(test_target.target());
        out.print("Hello, World!").unwrap();
        assert_eq!(test_target.to_string(), "Hello, World!\n")
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
        ).unwrap();
        assert_eq!(
            test_target.to_string(),
            "\u{1b}[0m\u{1b}[32m\u{1b}[44mhello\u{1b}[0m\n"
        )
    }
}
