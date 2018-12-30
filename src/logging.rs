use crate::{components::text, span};
use log::Level;

#[derive(Serialize)]
pub struct LogMessage {
    level: log::Level,
    path: String,
    message: String,
    // TODO: time: chrono::Local,
}

impl crate::Render for LogMessage {
    crate::render_for_humans!(self -> [
        match self.level {
            Level::Error => span!(fg = "red", [text("ERROR"),]),
            Level::Warn => span!(fg = "yellow", [text("WARN "),]),
            Level::Info => span!(fg = "blue", [text("INFO "),]),
            Level::Debug => span!(fg = "cyan", [text("DEBUG "),]),
            Level::Trace => span!(fg = "white", [text("DEBUG  "),]),
        },
        if self.path.is_empty() {
            span!([text(": "),])
        } else {
            span!([
                text(" <"),
                text(&self.path),
                text("> "),
            ])
        },
        text(&self.message),
    ]);

    crate::render_json!();
}

impl log::Log for crate::Output {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.inner
            .lock()
            .ok()
            .and_then(|o| o.log_level)
            .map(|level| metadata.level() <= level)
            .unwrap_or(false)
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let _ = self.print(&LogMessage {
                level: record.level(),
                path: record
                    .module_path()
                    .map(|x| x.to_string())
                    .unwrap_or_default(),
                message: record.args().to_string(),
            });
        }
    }

    fn flush(&self) {
        let _ = self.flush();
    }
}
