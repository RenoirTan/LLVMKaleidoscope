//! Error types used in `kaleidoscope_lexer`.

use kaleidoscope_macro::impl_display;
use std::{error, fmt::Display};

/// The kind of error encountered.
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    FileIOError,
    InvalidChar,
    BadChar,
    InvalidToken,
    LexerFatal,
    Other,
}

impl_display!(ErrorKind);

/// A struct representing an error.
#[derive(Clone, Debug)]
pub struct Error {
    description: String,
    errorkind: ErrorKind,
}

impl Error {
    pub fn new(description: &dyn AsRef<str>, errorkind: ErrorKind) -> Self {
        Self {
            description: description.as_ref().to_string(),
            errorkind,
        }
    }

    pub fn from_err(err: &dyn Display, errorkind: ErrorKind) -> Self {
        Self {
            description: format!("{}", err),
            errorkind,
        }
    }
}

impl_display!(Error);

impl error::Error for Error {}

/// Custom result type for the error type defined here.
pub type Result<T> = std::result::Result<T, Error>;
