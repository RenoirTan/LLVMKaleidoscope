//! Helper functions


use std::io::{Result, Write};
use env_logger::fmt::Formatter;
use log::Record;


pub fn format_record(
    formatter: &mut Formatter,
    record: &Record<'_>
) -> Result<()> {
    writeln!(
        formatter,
        "{} : {} : {} - {}",
        record.level(),
        record.module_path().unwrap_or("<module>"),
        record.line().unwrap_or(0),
        record.args()
    )
}
