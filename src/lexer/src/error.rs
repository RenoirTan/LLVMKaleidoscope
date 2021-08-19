//! Error types used in `kaleidoscope_lexer`.

use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;

/// The kind of error encountered in this library.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// If an error was encountered when reading a stream, this error flag
    /// is raised.
    FileIOError,
    /// If an unrecognised character was encountered when producing a token,
    /// this error is raised.
    InvalidChar,
    /// If a character was found at the wrong place in a token.
    BadChar,
    /// If the lexer could not resolve a sequence of bytes into a token.
    InvalidToken,
    /// If a series of characters in the wrong order or combination results
    /// in an invalid token.
    InvalidCombo,
    /// If there are too many characters in a token.
    ExcessiveChars,
    /// If the lexer encounters an exception that cannot be dealt with (either
    /// due to a uncaught edge case or a workaround has not been found yet).
    LexerFatal,
    /// For miscellaneous errors too obscure to have its own category.
    Other,
}

impl_display!(ErrorKind);

/// The error type used in this library.
pub type Error = klerr::Error<ErrorKind>;

/// Custom result type for the error type defined here.
pub type Result<T> = klerr::Result<T, ErrorKind>;
