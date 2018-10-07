//! Human output

use std::sync::{Arc, Mutex};
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
use {Error, Target};

/// Construct a new human output target that writes to stdout
pub fn stdout() -> Result<Target, Error> {
    Ok(Target::Human(Arc::new(Mutex::new(Formatter::init_with(
        || Ok(StandardStream::stdout(ColorChoice::Auto)),
    )?))))
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
        self.send(Message::Write(data.into()));
        Ok(())
    }

    /// Set color
    pub fn set_color(&self, spec: &ColorSpec) -> Result<(), Error> {
        self.send(Message::SetColor(spec.clone()));
        Ok(())
    }

    /// Reset color and styling
    pub fn reset(&self) -> Result<(), Error> {
        self.send(Message::ResetStyle);
        Ok(())
    }

    /// Immediately write all buffered output
    pub fn flush(&self) -> Result<(), Error> {
        self.send(Message::Flush);

        match self.inner.receiver.recv() {
            Some(Response::Flushed) => Ok(()),
            msg => Err(Error::WorkerError(format!("unexpected message {:?}", msg))),
        }
    }

    fn send(&self, msg: Message) {
        self.inner.sender.send(msg);
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
                    response_sender.send(Response::StartedSuccessfully);
                    buf
                }
                Err(e) => {
                    response_sender.send(Response::Error(e));
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
                    Some(Message::Write(data)) => {
                        let _ = buffer.write_all(&data).map_err(maybe_log_error!());
                    }
                    Some(Message::SetColor(data)) => {
                        let _ = buffer.set_color(&data).map_err(maybe_log_error!());
                    }
                    Some(Message::ResetStyle) => {
                        let _ = buffer.reset().map_err(maybe_log_error!());
                    }
                    Some(Message::Flush) => {
                        let _ = buffer.flush().map_err(maybe_log_error!());
                        response_sender.send(Response::Flushed);
                    }
                    Some(Message::Exit) | None => {
                        break;
                    }
                };
            }
        });

        match response_receiver.recv() {
            Some(Response::Error(error)) => Err(error),
            Some(Response::StartedSuccessfully) => Ok(InternalFormatter {
                worker: Some(worker),
                sender: message_sender,
                receiver: response_receiver,
            }),
            msg => Err(Error::WorkerError(format!("unexpected message {:?}", msg))),
        }
    }
}

impl Drop for InternalFormatter {
    fn drop(&mut self) {
        self.sender.send(Message::Exit);
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
///     render_for_humans!(self -> [
///         // compose output components
///         text("Important notice from "),
///         // refer to struct fields using the name you specified above
///         text(&self.author), newline(),
///         text("> "), text(&self.body),
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
    ($self:ident -> [$($item:expr,)*]) => {
        fn render_for_humans(&$self, fmt: &mut $crate::human::Formatter) -> Result<(), $crate::Error> {
            let span = span!([ $( $item, )* ]);
            span.render_for_humans(fmt)?;
            Ok(())
        }
    };
}

mod test_helper {
    use super::Formatter;
    use std::sync::{Arc, Mutex};
    use termcolor::Buffer;
    use {test_buffer::TestBuffer, Target};

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
    /// extern crate output;
    ///
    /// fn main() -> Result<(), output::Error> {
    ///     let test_target = output::human::test();
    ///     let mut out = output::new().add_target(test_target.target());
    ///     out.print(output::components::text("lorem ipsum"))?;
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
            Target::Human(Arc::new(Mutex::new(self.formatter())))
        }

        pub fn to_string(&self) -> String {
            let target = self.buffer.0.clone();
            let buffer = target.read().unwrap();
            String::from_utf8_lossy(buffer.as_slice()).to_string()
        }
    }
}
