//! Human output

use crate::{Error, Target};
use std::sync::Arc;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Construct a new human output target that writes to stdout
pub fn stdout() -> Result<Target, Error> {
    let formatter = Formatter::init_with(|| Ok(StandardStream::stdout(ColorChoice::Auto)))?;
    Ok(Target::human(formatter))
}

pub use self::test_helper::{test, test_with_color};

/// Human output formatter
#[derive(Clone)]
pub struct Formatter {
    inner: Arc<InternalFormatter>,
}

impl Formatter {
    pub(crate) fn init_with<W: WriteColor, F: FnOnce() -> Result<W, Error> + Send + 'static>(
        init: F,
    ) -> Result<Self, Error> {
        Ok(Formatter {
            inner: Arc::new(InternalFormatter::init_with(init)?),
        })
    }

    /// Write to target
    pub fn write<D: Into<Vec<u8>>>(&self, data: D) -> Result<(), Error> {
        self.send(Message::Write(data.into()))?;
        Ok(())
    }

    /// Set color
    pub fn set_color(&self, spec: &ColorSpec) -> Result<(), Error> {
        self.send(Message::SetColor(spec.clone()))?;
        Ok(())
    }

    /// Reset color and styling
    pub fn reset(&self) -> Result<(), Error> {
        self.send(Message::ResetStyle)?;
        Ok(())
    }

    /// Immediately write all buffered output
    pub fn flush(&self) -> Result<(), Error> {
        self.send(Message::Flush)?;

        match self.inner.receiver.recv() {
            Ok(Response::Flushed) => Ok(()),
            msg => Err(Error::worker_error(format!("unexpected message {:?}", msg))),
        }
    }

    fn send(&self, msg: Message) -> Result<(), Error> {
        self.inner.sender.send(msg)?;
        Ok(())
    }
}

use crossbeam_channel as channel;
use std::thread;

struct InternalFormatter {
    sender: channel::Sender<Message>,
    receiver: channel::Receiver<Response>,
    // Only an option so we can `take` this in `Drop::drop`
    worker: Option<thread::JoinHandle<()>>,
}

impl InternalFormatter {
    pub(crate) fn init_with<W: WriteColor, F: FnOnce() -> Result<W, Error> + Send + 'static>(
        init: F,
    ) -> Result<Self, Error> {
        let (message_sender, message_receiver) = channel::unbounded();
        let (response_sender, response_receiver) = channel::bounded(0);

        let worker = thread::spawn(move || {
            let mut buffer = match init() {
                Ok(buf) => {
                    let _ = response_sender.send(Response::StartedSuccessfully);
                    buf
                }
                Err(e) => {
                    let _ = response_sender.send(Response::Error(e));
                    return;
                }
            };

            macro_rules! maybe_log_error {
                () => {
                    |e| {
                        if cfg!(debug_assertions) {
                            eprintln!("{}", e)
                        } else {
                            ()
                        }
                    }
                };
            }

            loop {
                match message_receiver.recv() {
                    Ok(Message::Write(data)) => {
                        let _ = buffer.write_all(&data).map_err(maybe_log_error!());
                    }
                    Ok(Message::SetColor(data)) => {
                        let _ = buffer.set_color(&data).map_err(maybe_log_error!());
                    }
                    Ok(Message::ResetStyle) => {
                        let _ = buffer.reset().map_err(maybe_log_error!());
                    }
                    Ok(Message::Flush) => {
                        let _ = buffer.flush().map_err(maybe_log_error!());
                        let _ = response_sender.send(Response::Flushed);
                    }
                    Ok(Message::Exit) | Err(_) => {
                        break;
                    }
                };
            }
        });

        match response_receiver.recv() {
            Ok(Response::Error(error)) => Err(error),
            Ok(Response::StartedSuccessfully) => Ok(InternalFormatter {
                worker: Some(worker),
                sender: message_sender,
                receiver: response_receiver,
            }),
            msg => Err(Error::worker_error(format!("unexpected message {:?}", msg))),
        }
    }
}

impl Drop for InternalFormatter {
    fn drop(&mut self) {
        let _ = self.sender.send(Message::Exit);
        // TODO: Docs say this may panic, so have a look at how to deal with that.
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[derive(Debug)]
enum Message {
    Write(Vec<u8>),
    SetColor(ColorSpec),
    ResetStyle,
    Flush,
    Exit,
}

#[derive(Debug)]
enum Response {
    StartedSuccessfully,
    Error(Error),
    Flushed,
}

/// Shorthand for writing the `render_for_humans` method of the `Render` trait
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate convey;
/// #[macro_use] extern crate serde_derive;
///
/// use convey::{components::{text, newline}, Render};
///
/// #[derive(Serialize)]
/// struct Message {
///     author: String,
///     body: String,
/// }
///
/// impl Render for Message {
///     // we need to explicitly pass `self` here, similar to regular methods
///     render_for_humans!(self -> [
///         // compose output components
///         text("Important notice from "),
///         text(&self.author), newline(),
///         text("> "), text(&self.body),
///     ]);
///
///     // see `json` module
///     render_json!();
/// }
///
/// fn main() -> Result<(), convey::Error> {
///     let mut out = convey::new().add_target(convey::human::stdout()?)?;
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
            let span = $crate::span!([ $( $item, )* ]);
            span.render_for_humans(fmt)?;
            Ok(())
        }
    };
}

mod test_helper {
    use super::Formatter;
    use crate::{test_buffer::TestBuffer, Target};
    use termcolor::Buffer;

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
    /// extern crate convey;
    ///
    /// fn main() -> Result<(), convey::Error> {
    ///     let test_target = convey::human::test();
    ///     let mut out = convey::new().add_target(test_target.target())?;
    ///     out.print(convey::components::text("lorem ipsum"))?;
    ///     out.flush()?;
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
            let buffer = self.buffer.clone();
            Formatter::init_with(|| Ok(buffer)).unwrap()
        }

        pub fn target(&self) -> Target {
            Target::human(self.formatter())
        }
    }

    impl ::std::fmt::Display for TestTarget {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            let target = self.buffer.0.clone();
            let buffer = target.read().unwrap();
            write!(f, "{}", String::from_utf8_lossy(buffer.as_slice()))
        }
    }
}
