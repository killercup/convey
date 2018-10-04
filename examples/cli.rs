extern crate output;
extern crate failure;

use output::{json, human, components::{text}};
use std::fs::File;

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add(json::file("foo.log")?)
        .add(human::stdout()?);

    let x = 42;

    out.print(&text(x.to_string()));

    Ok(())
}