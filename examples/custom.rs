extern crate failure;
#[macro_use]
extern crate output;
#[macro_use]
extern crate serde_derive;

use output::{
    components::{span, text},
    human, json,
};

fn main() -> Result<(), failure::Error> {
    let mut out = output::new()
        .add_target(json::file("target/foo.log")?)
        .add_target(human::stdout()?);

    #[derive(Serialize)]
    struct ErrorMessage {
        code: i32,
        name: String,
        message: String,
    }

    use failure::Error;
    use output::RenderOutput;

    impl RenderOutput for ErrorMessage {
        fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
            span()
                .add_item(
                    span()
                        .fg("white")?
                        .bg("black")?
                        .add_item(text(self.code.to_string()))
                        .add_item(text(" ")),
                )
                .add_item(
                    span()
                        .fg("red")?
                        .bg("black")?
                        .add_item(text(self.name.clone())),
                )
                .add_item(text("\n> "))
                .add_item(text(self.message.clone()))
                .render_for_humans(fmt)?;

            Ok(())
        }

        render_json!();
    }

    out.print(&ErrorMessage {
        code: 42,
        name: String::from("error"),
        message: String::from("Oh god no"),
    })?;

    Ok(())
}
