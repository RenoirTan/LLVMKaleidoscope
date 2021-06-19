//! Error types used in `kaleidoscope_lexer`.

use std::error;
use kaleidoscope_macro::impl_display;

/// The kind of error encountered.
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    FileIOError,
    Other
}

impl_display!(ErrorKind);

/// A struct representing an error.
#[derive(Clone, Debug)]
pub struct Error {
    description: String,
    errorkind: ErrorKind
}

impl Error {
    pub fn new(description: String, errorkind: ErrorKind) -> Self {
        Self {description, errorkind}
    }
}

impl_display!(Error);

impl error::Error for Error {}

/// Custom result type for the error type defined here.
pub type Result<T> = std::result::Result<T, Error>;
