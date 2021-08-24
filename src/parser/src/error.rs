//! Error module defining the types used when handling errors in this crate.

use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;

/// The types of errors you may encounter in LLVMKaleidoscope-Parser.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    LexerError,
    ParsingError,
    SyntaxError,
    Other
}

impl_display!(ErrorKind);

pub type Error = klerr::Error<ErrorKind>;

pub type Result<T> = klerr::Result<T, ErrorKind>;
