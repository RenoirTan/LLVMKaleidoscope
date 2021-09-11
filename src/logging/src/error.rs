//! Module for error handling in this sub-crate.


use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;

/// The enum that classifies the type of error encountered by the logger.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    FormatterError,
    LoggerError,
    Other
}

impl_display!(ErrorKind);

/// The error type used by this crate.
pub type Error = klerr::Error<ErrorKind>;

/// The result type used by this crate.
pub type Result<T> = klerr::Result<T, ErrorKind>;
