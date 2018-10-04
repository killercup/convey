extern crate failure;
extern crate output;

use output::{
    components::{color, text},
    human, json,
};

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add(json::file("target/foo.log")?)
        .add(human::stdout()?);

    let x = 42;

    out.print(&text(x.to_string()))?;

    out.print(&color().fg("blue")?.bg("yellow")?.add_item(text("hello")))?;

    Ok(())
}
