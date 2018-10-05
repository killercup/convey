use failure::Error;
use termcolor::{ColorSpec, WriteColor};
use {human, json, RenderOutput};

pub fn span() -> Span {
    Span::default()
}

#[derive(Default)]
pub struct Span {
    items: Vec<Box<RenderOutput>>,
    fg: Option<::termcolor::Color>,
    bg: Option<::termcolor::Color>,
}

impl Span {
    pub fn add_item<T: RenderOutput + 'static>(mut self, item: T) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn fg(mut self, color: &str) -> Result<Self, Error> {
        self.fg = Some(color.parse()?);
        Ok(self)
    }

    pub fn bg(mut self, color: &str) -> Result<Self, Error> {
        self.bg = Some(color.parse()?);
        Ok(self)
    }
}

impl RenderOutput for Span {
    fn render_for_humans(&self, fmt: &mut human::Formatter) -> Result<(), Error> {
        fmt.writer
            .set_color(ColorSpec::new().set_fg(self.fg).set_bg(self.bg))?;
        for item in &self.items {
            item.render_for_humans(fmt)?;
        }
        fmt.writer.reset()?;
        Ok(())
    }

    fn render_json(&self, fmt: &mut json::Formatter) -> Result<(), Error> {
        for item in &self.items {
            item.render_json(fmt)?;
        }
        Ok(())
    }
}
