use std::io;
use std::sync::{Arc, RwLock};
use termcolor::{Buffer, ColorSpec, WriteColor};

#[derive(Clone)]
pub(crate) struct TestBuffer(pub(crate) Arc<RwLock<Buffer>>);

impl From<Buffer> for TestBuffer {
    fn from(x: Buffer) -> Self {
        TestBuffer(Arc::new(RwLock::new(x)))
    }
}

impl io::Write for TestBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let target = self.0.clone();
        let mut buffer = target.write().unwrap();
        buffer.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        let target = self.0.clone();
        let mut buffer = target.write().unwrap();
        buffer.flush()
    }
}

impl WriteColor for TestBuffer {
    fn supports_color(&self) -> bool {
        let target = self.0.clone();
        let buffer = target.read().unwrap();
        buffer.supports_color()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> Result<(), io::Error> {
        let target = self.0.clone();
        let mut buffer = target.write().unwrap();
        buffer.set_color(spec)
    }

    fn reset(&mut self) -> Result<(), io::Error> {
        let target = self.0.clone();
        let mut buffer = target.write().unwrap();
        buffer.reset()
    }
}
