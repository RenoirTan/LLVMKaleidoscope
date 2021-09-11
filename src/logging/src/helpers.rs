//! Helper functions used by this crate.


use std::io::{Result, Write};

use env_logger::fmt::Formatter;
use log::Record;

/// Format a record from the logger into a string to be used by the
/// [`Formatter`].
pub fn format_record(formatter: &mut Formatter, record: &Record<'_>) -> Result<()> {
    writeln!(
        formatter,
        "{} : {} : {} - {}",
        record.level(),
        record.module_path().unwrap_or("<module>"),
        record.line().unwrap_or(0),
        record.args()
    )
}
