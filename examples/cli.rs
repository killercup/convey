extern crate output;
extern crate failure;

use output::{targets::{Json, Human}, Text};
use std::fs::File;

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add(Json::file("foo.log")?)
        .add(Human::stdout()?)
        .build()?;

    let x = 42;

    out.print(&Text(x.to_string()));

    Ok(())
}