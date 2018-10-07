//! JSON output

use serde::Serialize;
use serde_json::to_vec as write_json;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use {Error, Target};

/// Create a new JSON output that writes to a file
pub fn file<T: AsRef<Path>>(name: T) -> Result<Target, Error> {
    let path = name.as_ref().to_path_buf();
    Ok(Target::Json(Arc::new(Mutex::new(Formatter::init_with(
        move || {
            use std::fs::{File, OpenOptions};
            use std::io::BufWriter;

            let target = if path.exists() {
                let mut f = OpenOptions::new().write(true).append(true).open(&path)?;
                f.write_all(b"\n")?;

                f
            } else {
                File::create(&path)?
            };

            Ok(BufWriter::new(target))
        },
    )?))))
}

pub use self::test_helper::test;

/// JSON formatter
#[derive(Clone)]
pub struct Formatter {
    inner: Arc<InternalFormatter>,
}

impl Formatter {
    pub(crate) fn init_with<W: Write, F: FnOnce() -> Result<W, Error> + Send + 'static>(
        init: F,
    ) -> Result<Self, Error> {
        Ok(Formatter {
            inner: Arc::new(InternalFormatter::init_with(init)?),
        })
    }

    /// Write a serializable item to the JSON formatter
    pub fn write<T: Serialize>(&self, item: &T) -> Result<(), Error> {
        self.send(Message::Write(write_json(item)?));
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

    /// Write a separator after a record
    pub(crate) fn write_separator(&mut self) -> Result<(), Error> {
        self.send(Message::Write(vec![b'\n']));
        Ok(())
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
    pub(crate) fn init_with<W: Write, F: FnOnce() -> Result<W, Error> + Send + 'static>(
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
    Flush,
    Exit,
}

#[derive(Debug)]
enum Response {
    StartedSuccessfully,
    Error(Error),
    Flushed,
}

/// Shorthand for writing the `render_json` method of the `Render`  trait
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate output;
/// #[macro_use] extern crate serde_derive;
///
/// #[derive(Serialize)]
/// struct Message {
///     author: String,
///     body: String,
/// }
///
/// impl output::Render for Message {
///     // Look at the `human` module if you care about those meat bags.
///     render_for_humans!(this -> []);
///
///     // We're lucky, or type implements `Serialize`. Nothing to do!
///     render_json!();
/// }
///
/// fn main() -> Result<(), output::Error> {
///     let mut out = output::new().add_target(output::human::stdout()?);
///     out.print(Message { author: "Pascal".into(), body: "Lorem ipsum dolor".into() })?;
///     Ok(())
/// }
/// ```
// TODO: Add mode for stuff like `render_json!(this -> { "success": true, "excerpt": this.body.lines().next() });`
#[macro_export]
macro_rules! render_json {
    () => {
        fn render_json(&self, fmt: &mut $crate::json::Formatter) -> Result<(), $crate::Error> {
            fmt.write(self)?;
            Ok(())
        }
    }
}

mod test_helper {
    use super::Formatter;
    use std::sync::{Arc, Mutex};
    use termcolor::Buffer;
    use test_buffer::TestBuffer;
    use Target;

    /// Create a test output target
    ///
    /// **Note:** This is intended for usage in tests; this and the methods you can call out in will
    /// often just panic instead of return `Result`s.
    ///
    /// # Usage
    ///
    /// ```rust
    /// #[macro_use] extern crate output;
    /// #[macro_use] extern crate serde_derive;
    ///
    /// fn main() -> Result<(), output::Error> {
    ///     let test_target = output::json::test();
    ///     let mut out = output::new().add_target(test_target.target());
    ///
    ///     #[derive(Serialize)]
    ///     struct Message {
    ///         author: String,
    ///         body: String,
    ///     }
    ///     impl output::Render for Message {
    ///         render_for_humans!(this -> []);
    ///         render_json!();
    ///     }
    ///
    ///     out.print(Message { author: "Pascal".into(), body: "Lorem ipsum dolor".into() })?;
    ///     out.flush()?;
    ///
    ///     assert_eq!(
    ///         test_target.to_string(),
    ///         "{\"author\":\"Pascal\",\"body\":\"Lorem ipsum dolor\"}\n\n",
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn test() -> TestTarget {
        TestTarget {
            buffer: Buffer::no_color().into(),
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
            Target::Json(Arc::new(Mutex::new(self.formatter())))
        }

        pub fn to_string(&self) -> String {
            let target = self.buffer.0.clone();
            let buffer = target.read().unwrap();
            String::from_utf8_lossy(buffer.as_slice()).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::*;
    use json;
    use predicates::prelude::*;

    type Res = Result<(), ::failure::Error>;

    #[test]
    fn creates_a_new_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        log_file.assert(predicate::path::missing());

        {
            // drop to flush file write buffer
            let _target = json::file(log_file.path())?;
        }

        log_file.assert(predicate::path::exists());

        Ok(())
    }

    #[test]
    fn doesnt_truncate_existing_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        let intitial_content = "{\"success\":true}\n";

        log_file.write_str(intitial_content)?;
        log_file.assert(predicate::path::exists());

        {
            // drop to flush file write buffer
            let _target = json::file(log_file.path())?;
        }

        log_file.assert(
            predicate::str::contains(intitial_content)
                .from_utf8()
                .from_file_path(),
        );

        Ok(())
    }

    #[test]
    fn appends_to_existing_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        let intitial_content = "{\"success\":true}\n";

        log_file.write_str(intitial_content)?;
        log_file.assert(predicate::path::exists());

        let target = json::file(log_file.path())?;
        let mut output = ::new().add_target(target);
        output.print("wtf")?;
        output.flush()?;

        log_file.assert(
            predicate::str::ends_with("\"wtf\"")
                .trim()
                .from_utf8()
                .from_file_path(),
        );

        Ok(())
    }

    #[test]
    fn appends_newline_to_existing_file() -> Res {
        let dir = TempDir::new()?;
        let log_file = dir.child("log.json");
        let intitial_content = "{\"success\":true}"; // <- no newline at the end

        log_file.write_str(intitial_content)?;
        log_file.assert(predicate::path::exists());

        {
            // drop to flush file write buffer
            let _target = json::file(log_file.path())?;
        }

        log_file.assert(predicate::str::ends_with("\n").from_utf8().from_file_path());

        Ok(())
    }
}
