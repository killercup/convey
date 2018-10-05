use std::io::Write;
use {human, json, Error, RenderOutput};

pub fn text<T: AsRef<str>>(input: T) -> Text {
    Text(input.as_ref().to_string())
}

pub fn newline() -> Text {
    text("\n")
}

#[derive(Clone, Serialize)]
pub struct Text(String);

impl RenderOutput for Text {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.writer.write_all(self.0.as_bytes())?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        fmt.write(self)?;
        Ok(())
    }
}
