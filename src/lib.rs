//! Easily output stuff for humans and machines alike
//!
//! # Examples
//!
//! ```rust
//! extern crate convey;
//!
//! fn main() -> Result<(), convey::Error> {
//!     let mut out = convey::new().add_target(convey::human::stdout()?);
//!     out.print(convey::components::text("hello world!"))?;
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
#[derive(Default, Clone)]
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

use std::sync::{Arc, Mutex};

/// Known targets to write to
#[derive(Clone)]
pub struct Target {
    inner: InnerTarget,
}

impl Target {
    /// Human readable output
    ///
    /// Will mostly be (unstructured) text, optionally with formatting.
    pub(crate) fn human(f: human::Formatter) -> Self {
        Target {
            inner: InnerTarget::Human(Arc::new(Mutex::new(f))),
        }
    }

    /// JSON output
    ///
    /// Machines like this.
    pub(crate) fn json(f: json::Formatter) -> Self {
        Target {
            inner: InnerTarget::Json(Arc::new(Mutex::new(f))),
        }
    }
}

#[derive(Clone)]
enum InnerTarget {
    Human(Arc<Mutex<human::Formatter>>),
    Json(Arc<Mutex<json::Formatter>>),
}

mod error;
pub use error::Error;

impl Output {
    /// Print some item to the currently active output targets
    pub fn print<O: Render>(&mut self, item: O) -> Result<(), Error> {
        for target in &mut self.targets {
            match &target.inner {
                InnerTarget::Human(fmt) => {
                    let mut fmt = fmt.lock().map_err(|e| Error::sync_error(&e))?;
                    item.render_for_humans(&mut *fmt)?;
                    fmt.write("\n")?;
                }
                InnerTarget::Json(fmt) => {
                    let mut fmt = fmt.lock().map_err(|e| Error::sync_error(&e))?;
                    item.render_json(&mut *fmt)?;
                    fmt.write_separator()?;
                }
            }
        }

        Ok(())
    }

    /// Immediately write all buffered output
    pub fn flush(&self) -> Result<(), Error> {
        for target in &self.targets {
            match &target.inner {
                InnerTarget::Human(fmt) => {
                    let mut fmt = fmt.lock().map_err(|e| Error::sync_error(&e))?;
                    fmt.flush()?;
                }
                InnerTarget::Json(fmt) => {
                    let mut fmt = fmt.lock().map_err(|e| Error::sync_error(&e))?;
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
/// # extern crate convey;
/// # use convey::{human, components::text};
/// # fn main() -> Result<(), convey::Error> {
/// # let test_target = human::test();
/// let mut out = convey::new().add_target(test_target.target());
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
/// # extern crate convey;
/// # use convey::human;
/// # fn main() -> Result<(), convey::Error> {
/// # let test_target = human::test();
/// let mut out = convey::new().add_target(test_target.target());
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

/// Render a string
///
/// # Examples
///
/// ```rust
/// # extern crate convey;
/// # use convey::human;
/// # fn main() -> Result<(), convey::Error> {
/// # let test_target = human::test();
/// let mut out = convey::new().add_target(test_target.target());
/// out.print(String::from("Hello, World!"))?;
/// # out.flush()?;
/// # assert_eq!(test_target.to_string(), "Hello, World!\n");
/// # Ok(()) }
/// ```
impl<'a> Render for String {
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
