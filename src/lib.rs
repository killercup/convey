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
extern crate crossbeam_channel;
extern crate serde;
extern crate termcolor;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
extern crate assert_fs;
#[cfg(test)]
extern crate predicates;

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

#[test]
fn assert_output_is_sync_and_send() {
    fn assert_both<T: Send + Sync>() {}
    assert_both::<Output>();
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
                    fmt.write("\n")?;
                }
                Target::Json(fmt) => {
                    item.render_json(fmt)?;
                    fmt.write_separator()?;
                }
            }
        }

        Ok(())
    }

    /// Immediately write all buffered output
    pub fn flush(&self) -> Result<(), Error> {
        for target in &self.targets {
            match target {
                Target::Human(fmt) => {
                    fmt.flush()?;
                }
                Target::Json(fmt) => {
                    fmt.flush()?;
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

/// Render automatically works with references
///
/// # Examples
///
/// ```rust
/// # extern crate output;
/// # use output::{human, components::text};
/// # fn main() -> Result<(), output::Error> {
/// # let test_target = human::test();
/// let mut out = output::new().add_target(test_target.target());
/// out.print(text("owned element"))?;
/// out.print(&text("reference to an element"))?;
/// # out.flush()?;
/// # assert_eq!(test_target.to_string(), "owned element\nreference to an element\n");
/// # Ok(()) }
/// ```
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

/// Render a string slice
///
/// # Examples
///
/// ```rust
/// # extern crate output;
/// # use output::human;
/// # fn main() -> Result<(), output::Error> {
/// # let test_target = human::test();
/// let mut out = output::new().add_target(test_target.target());
/// out.print("Hello, World!")?;
/// # out.flush()?;
/// # assert_eq!(test_target.to_string(), "Hello, World!\n");
/// # Ok(()) }
/// ```
impl<'a> Render for &'a str {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.write(self.as_bytes())?;
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
