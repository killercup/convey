extern crate failure;
extern crate output;
#[macro_use]
extern crate output_derive;
#[macro_use]
extern crate serde_derive;

use output::{human, json};

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add_target(json::file("target/foo.log")?)
        .add_target(human::stdout()?);

    #[derive(Serialize, RenderOutput)]
    struct ErrorMessage {
        code: i32,
        name: String,
        message: String,
    }

    out.print(&ErrorMessage {
        code: 42,
        name: String::from("info"),
        message: String::from("Derive works"),
    })?;

    out.print(&ErrorMessage {
        code: 0,
        name: String::from("okay"),
        message: String::from("Thanks for stopping by"),
    })?;

    Ok(())
}
