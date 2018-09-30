use termcolor::WriteColor;

use targets::{Human, HumanError, HumanOutput, Json, JsonError, JsonOutput, OutputTarget};

// fixme(killercup): how to deal with this associated type here?
// refactor trait to return a known (dynamic) type instead?
pub struct Color<'f>(
    pub Vec<Box<dyn OutputTarget<'f>>>,
    pub termcolor::ColorSpec,
);

impl<'f> HumanOutput<'f> for Color<'f> {
    fn human_output(&self, f: &mut Human<'f>) -> Result<(), HumanError> {
        f.set_color(&self.1)?;
        self.0.iter().try_for_each(|x| x.human_output(f))?;
        f.reset()?;
        Ok(())
    }
}

impl<'f> JsonOutput<'f> for Color<'f> {
    fn json_output(&self, f: &mut Json<'f>) -> Result<(), JsonError> {
        self.0.iter().try_for_each(|x| x.json_output(f))?;
        Ok(())
    }
}