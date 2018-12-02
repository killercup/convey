extern crate failure;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate convey;

use structopt::StructOpt;
use convey::{
    components::{newline, text},
    human, json,
};

/// Demo stuff
#[derive(StructOpt)]
struct Cli {
    /// Output JSON instead of human readable messages
    #[structopt(long = "json")]
    json: bool,
}

fn main() -> Result<(), failure::Error> {
    let args = Cli::from_args();
    let mut out = if args.json {
        convey::new().add_target(json::stdout()?)
    } else {
        convey::new().add_target(human::stdout()?)
    };

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

    #[derive(Serialize)]
    struct ErrorMessage {
        code: i32,
        name: String,
        message: String,
    }

    impl convey::Render for ErrorMessage {
        render_for_humans!(self -> [
            span!(fg = "white", bg = "black", [text(self.code.to_string()), text(" "),]),
            span!(fg = "red", bg = "black", [text(&self.name),]),
            newline(),
            text("> "),
            text(&self.message),
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
