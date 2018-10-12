extern crate failure;
#[macro_use]
extern crate convey;

use convey::{
    components::{newline, text},
    human, json,
};

fn main() -> Result<(), failure::Error> {
    let mut out = convey::new()
        .add_target(json::file("target/foo.log")?)
        .add_target(human::stdout()?);

    let x = 42;

    out.print(text(x.to_string()))?;
    out.print(span!([
        span!(fg = "blue", bg = "yellow", ["colorful text",]),
        newline(),
        span!(bold = true, ["bold text",]),
        newline(),
        span!(underline = true, ["underlined text",]),
        newline(),
        span!(intense = true, ["intense text",]),
    ]))?;

    Ok(())
}
