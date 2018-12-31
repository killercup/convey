//! Easily output stuff for humans and machines alike
//!
//! # Examples
//!
//! ```rust
//! extern crate convey;
//!
//! fn main() -> Result<(), convey::Error> {
//!     let mut out = convey::new().add_target(convey::human::stdout()?)?;
//!     out.print(convey::components::text("hello world!"))?;
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

extern crate crossbeam_channel;
extern crate failure;
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
#[cfg(test)]
extern crate assert_fs;
#[cfg(feature = "log")]
extern crate log;
#[cfg(test)]
extern crate predicates;

/// Create a new output
pub fn new() -> Output {
    Output::default()
}

/// Structure holding your output targets
#[derive(Default, Clone)]
pub struct Output {
    inner: Arc<Mutex<InnerOutput>>,
}

#[derive(Default, Clone)]
struct InnerOutput {
    targets: Vec<Target>,
    #[cfg(feature = "log")]
    log_level: Option<log::Level>,
}

impl Output {
    /// Add a target to output to
    pub fn add_target(self, target: Target) -> Result<Self, Error> {
        {
            let mut o = self.inner.lock().map_err(|e| Error::sync_error(&e))?;
            o.targets.push(target);
        }
        Ok(self)
    }

    /// Initializes the global logger with an `Output` instance with
    /// `max_log_level` set to a specific log level.
    ///
    /// ```
    /// # extern crate convey;
    /// # fn main() -> Result<(), convey::Error> {
    /// let output = convey::new()
    ///     .add_target(convey::human::stdout()?)?
    ///     .use_as_logger(log::Level::Debug)?;
    ///
    /// log::info!("welcome");
    /// log::error!("oh noes");
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "log")]
    pub fn use_as_logger(self, level: log::Level) -> Result<Self, Error> {
        {
            let mut o = self.inner.lock().map_err(|e| Error::sync_error(&e))?;
            o.log_level = Some(level);
        }
        log::set_boxed_logger(Box::new(self.clone()))?;
        log::set_max_level(level.to_level_filter());
        Ok(self)
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
    pub fn print<O: Render>(&self, item: O) -> Result<(), Error> {
        let mut o = self.inner.lock().map_err(|e| Error::sync_error(&e))?;
        for target in &mut o.targets {
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
        let o = self.inner.lock().map_err(|e| Error::sync_error(&e))?;
        for target in &o.targets {
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
/// let mut out = convey::new().add_target(test_target.target())?;
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
/// let mut out = convey::new().add_target(test_target.target())?;
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
/// let mut out = convey::new().add_target(test_target.target())?;
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

#[cfg(feature = "log")]
mod logging;

mod test_buffer;
