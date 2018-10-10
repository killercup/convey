extern crate failure;
extern crate output;

use output::{
    components::{span, text},
    human, json,
};

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add_target(json::file("target/foo.log")?)
        .add_target(human::stdout()?);

    let x = 42;

    out.print(&text(x.to_string()))?;

    out.print(
        &span()
            .fg("blue")?
            .bg("yellow")?
            .underline(true)
            .bold(true)
            .intense(true)
            .add_item(text("hello")),
    )?;

    Ok(())
}
