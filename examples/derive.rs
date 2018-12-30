extern crate convey;
extern crate failure;
#[macro_use]
extern crate convey_derive;
#[macro_use]
extern crate serde_derive;

use convey::{human, json};

fn main() -> Result<(), failure::Error> {
    let out = convey::new()
        .add_target(json::file("target/foo.log")?)?
        .add_target(human::stdout()?)?;

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
