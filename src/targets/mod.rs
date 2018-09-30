pub(crate) mod human;
pub(crate) mod json;

pub use self::human::Formatter as Human;
pub use self::human::Output as HumanOutput;
pub use self::human::Error as HumanError;

pub use self::json::Formatter as Json;
pub use self::json::Output as JsonOutput;
pub use self::json::Error as JsonError;

//enum Targets

pub trait Formatter {
    //    type Error;
    fn print(&mut self, item: &OutputTarget) -> Result<(), ::failure::Error>;
}

impl<'f> Formatter for human::Formatter<'f> {
    //    type Error = human::Error;
    fn print(&mut self, item: &OutputTarget) -> Result<(), ::failure::Error> {
        item.human_output(&mut self)?;
        Ok(())
    }
}

impl<'f> Formatter for json::Formatter<'f> {
    //    type Error = json::Error;
    fn print(&mut self, item: &OutputTarget) -> Result<(), ::failure::Error> {
        item.json_output(&mut self)?;
        Ok(())
    }
}

pub trait OutputTarget<'f>: HumanOutput<'f> + JsonOutput<'f> {}

impl<'f, T> OutputTarget<'f> for T where T: HumanOutput<'f> + JsonOutput<'f> {}

