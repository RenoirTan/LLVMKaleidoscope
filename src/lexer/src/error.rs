//! Error types used in `kaleidoscope_lexer`.

use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;

/// The kind of error encountered.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    FileIOError,
    InvalidChar,
    BadChar,
    InvalidToken,
    InvalidCombo,
    ExcessiveChars,
    LexerFatal,
    Other,
}

impl_display!(ErrorKind);

pub type Error = klerr::Error<ErrorKind>;

/// Custom result type for the error type defined here.
pub type Result<T> = klerr::Result<T, ErrorKind>;
