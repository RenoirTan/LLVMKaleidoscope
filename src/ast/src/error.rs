//! Error types used in `kaleidoscope_ast`.

use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;

/// The kind of error encountered.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    WrongTokenKind,
    TypeCasting,
    Other
}

impl_display!(ErrorKind);

/// Special error type for this [`crate`].
pub type Error = klerr::Error<ErrorKind>;

/// Custom result type for the error type defined here.
pub type Result<T> = klerr::Result<T, ErrorKind>;
