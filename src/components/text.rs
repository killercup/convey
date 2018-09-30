use std::io::Write;

use targets::{Human, HumanError, HumanOutput, Json, JsonError, JsonOutput};

pub struct Text(pub String);

impl<'f> HumanOutput<'f> for Text {
    fn human_output(&self, f: &mut Human<'f>) -> Result<(), HumanError> {
        f.write(self.0.as_bytes())?;
        Ok(())
    }
}

impl<'f> JsonOutput<'f> for Text {
    fn json_output(&self, f: &mut Json<'f>) -> Result<(), JsonError> {
        f.write(&self.0)?;
        Ok(())
    }
}
