use failure::Error;
use std::io::Write;
use {human, json, RenderOutput};

pub fn text<T: Into<String>>(input: T) -> Text {
    Text(input.into())
}

#[derive(Clone, Serialize)]
pub struct Text(String);

impl RenderOutput for Text {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.writer.write(self.0.as_bytes())?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        fmt.write(self)?;
        Ok(())
    }
}
