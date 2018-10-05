use std::io::Write;
use {human, json, Error, RenderOutput};
use std::borrow::Cow;

pub fn text<T: Into<Cow<'static, str>>>(input: T) -> Text {
    Text(input.into())
}

#[derive(Clone, Serialize)]
pub struct Text(Cow<'static, str>);

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
