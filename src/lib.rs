extern crate termcolor;
extern crate serde;
extern crate serde_json;
extern crate failure;
#[macro_use] extern crate failure_derive;

pub mod targets;

mod components;
pub use components::color::Color;
pub use components::text::Text;
use std::marker::PhantomData;

pub struct Output<T> {
    targets: Vec<Box<targets::Formatter>>,
    state: PhantomData<T>,
}

pub struct OutputBuilding;
pub struct Outputting;

pub fn new() -> Output<OutputBuilding> {
    Output {
        targets: vec![],
        state: PhantomData,
    }
}

impl Output<OutputBuilding> {
    pub fn add<F: targets::Formatter + 'static>(mut self, new_output: F) -> Self {
        self.targets.push(Box::new(new_output));
        self
    }

    pub fn build(self) -> Result<Output<Outputting>, failure::Error> {
        Ok(Output {
            targets: self.targets,
            state: PhantomData,
        })
    }
}

impl Output<Outputting> {
    pub fn print<'f, 'i: 'f, O: targets::OutputTarget<'f>>(&mut self, item: &'i O) -> Result<(), failure::Error> {
        for target in &mut self.targets {
            //target.print(item)?;
        }

        Ok(())
    }
}
