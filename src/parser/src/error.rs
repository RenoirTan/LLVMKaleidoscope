use kaleidoscope_error as klerr;
use kaleidoscope_macro::impl_display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    Other
}

impl_display!(ErrorKind);

pub type Error = klerr::Error<ErrorKind>;

pub type Result<T> = klerr::Result<T, ErrorKind>;
