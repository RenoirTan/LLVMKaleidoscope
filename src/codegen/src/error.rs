use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    UndefinedNameError,
    UnknownOperationError,
    ValueError,
    TypeError,
    NotBasicValueError,
    CouldNotMakeFunctionError,
    BitWidthError,
    Other
}

impl_display!(ErrorKind);

pub type Error = klerr::Error<ErrorKind>;

pub type Result<T> = klerr::Result<T, ErrorKind>;
