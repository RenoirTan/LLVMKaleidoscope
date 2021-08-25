//! Helper functions


use ansi_term::{Colour, Style};
use log::{Level, Record};


pub(crate) fn level_to_style(level: &Level) -> Style {
    match *level {
        Level::Error => Style::new().on(Colour::Red).fg(Colour::Black),
        Level::Warn => Style::new().on(Colour::Yellow).fg(Colour::Black),
        Level::Info => Style::new().fg(Colour::White),
        Level::Debug
        | Level::Trace => Style::new().fg(Colour::White).dimmed()
    }
}


pub fn format_record(record: &Record<'_>) -> String {
    level_to_style(&record.level())
        .paint(format!(
            "{} : {} : {} - {}",
            record.level(),
            record.module_path().unwrap_or("<stdin>"),
            record.line().unwrap_or(0),
            record.args()
        ))
        .to_string()
}
