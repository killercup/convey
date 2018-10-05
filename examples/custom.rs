extern crate failure;
#[macro_use]
extern crate output;
#[macro_use]
extern crate serde_derive;

use output::{
    components::{newline, text},
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

    use output::RenderOutput;

    impl RenderOutput for ErrorMessage {
        render_for_humans!(this -> [
            span!(fg = "white", bg = "black", [text(this.code.to_string()), text(" "),]),
            span!(fg = "red", bg = "black", [text(&this.name),]),
            newline(),
            text("> "),
            text(&this.message),
        ]);

        render_json!();
    }

    out.print(&ErrorMessage {
        code: 42,
        name: String::from("error"),
        message: String::from("Oh god no"),
    })?;

    Ok(())
}
